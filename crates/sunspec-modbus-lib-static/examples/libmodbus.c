#include <errno.h>
#include <modbus/modbus.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#ifndef SUNSPEC_MODEL_103_ENABLED
#define SUNSPEC_MODEL_103_ENABLED
#endif

#ifndef SUNSPEC_MODEL_708_ENABLED
#define SUNSPEC_MODEL_708_ENABLED
#endif
#include "../libsunspecmodbus.h"


/// How often `randomise_voltages` refreshes the inverter's per-phase voltages, matching the
/// tokio-modbus example's VOLTAGE_REFRESH_INTERVAL.
#define VOLTAGE_REFRESH_INTERVAL_SECS 1

/// The device's entire mutable state. Everything else served below (manufacturer strings,
/// amperage, frequency, ...) is fixed, so there's nothing else to hold.
static atomic_uint_least16_t DEVICE_ADDRESS = 0;
static atomic_uint_least16_t VOLTAGE_AN = 0;
static atomic_uint_least16_t VOLTAGE_BN = 0;
static atomic_uint_least16_t VOLTAGE_CN = 0;

/// Model 1 (common) read/write callbacks. Every point but the device address is a fixed
/// value; the device address lives in `DEVICE_ADDRESS`. `context` is unused throughout, since
/// there's no per-adapter state to carry.
const char *manufacturer_callback(const void *context)
{
    (void)context;
    return "Cuprous";
}

const char *model_callback(const void *context)
{
    (void)context;
    return "Inverter 1";
}

const char *options_callback(const void *context)
{
    (void)context;
    return "opt_a_b_c";
}

const char *version_callback(const void *context)
{
    (void)context;
    return "v0.1";
}

const char *serial_number_callback(const void *context)
{
    (void)context;
    return "I-1";
}

uint16_t device_address_callback(const void *context)
{
    (void)context;
    return atomic_load(&DEVICE_ADDRESS);
}

void set_device_address_callback(uint16_t value, void *context)
{
    (void)context;
    atomic_store(&DEVICE_ADDRESS, value);
}

/// Model 103 (inverter) read callbacks. Every point but the per-phase voltages is a fixed
/// value; the voltages live in `VOLTAGE_AN` / `VOLTAGE_BN` / `VOLTAGE_CN`, refreshed on a
/// timer by `randomise_voltages`.
uint16_t amps_callback(const void *context)
{
    (void)context;
    return 0;
}

uint16_t amps_phase_a_callback(const void *context)
{
    (void)context;
    return 1;
}

uint16_t amps_phase_b_callback(const void *context)
{
    (void)context;
    return 1;
}

uint16_t amps_phase_c_callback(const void *context)
{
    (void)context;
    return 1;
}

int16_t a_sf_callback(const void *context)
{
    (void)context;
    return -1;
}

uint16_t phase_voltage_an_callback(const void *context)
{
    (void)context;
    return atomic_load(&VOLTAGE_AN);
}

uint16_t phase_voltage_bn_callback(const void *context)
{
    (void)context;
    return atomic_load(&VOLTAGE_BN);
}

uint16_t phase_voltage_cn_callback(const void *context)
{
    (void)context;
    return atomic_load(&VOLTAGE_CN);
}

int16_t v_sf_callback(const void *context)
{
    (void)context;
    return -1;
}

int16_t watts_callback(const void *context)
{
    (void)context;
    return 1;
}

int16_t w_sf_callback(const void *context)
{
    (void)context;
    return 1;
}

uint16_t hz_callback(const void *context)
{
    (void)context;
    return 1234;
}

int16_t hz_sf_callback(const void *context)
{
    (void)context;
    return -2;
}

uint32_t watt_hours_callback(const void *context)
{
    (void)context;
    return 1;
}

int16_t wh_sf_callback(const void *context)
{
    (void)context;
    return 0;
}

int16_t cabinet_temperature_callback(const void *context)
{
    (void)context;
    return 1;
}

int16_t tmp_sf_callback(const void *context)
{
    (void)context;
    return 0;
}

Model103St operating_state_callback(const void *context)
{
    (void)context;
    return Model103St_Standby;
}

uint32_t event1_callback(const void *context)
{
    (void)context;
    return 1;
}

uint32_t event_bitfield_2_callback(const void *context)
{
    (void)context;
    return 1;
}

/// Model 708 (DER high-voltage trip curves) read/write callbacks. Two stored curve sets
/// (`NCrvSet`) of three points (`NPt`) each, matching the tokio-modbus example's `CURVE_COUNT` /
/// `POINT_COUNT`. `Ena` and `AdptCrvReq` are the only writable points and round-trip through
/// `MODULE_ENABLED` / `ADOPT_CURVE_REQUEST`; every curve/point setter is left unset (NULL, i.e.
/// `None` on the Rust side), since this example doesn't support reconfiguring curves.
#define CURVE_COUNT 2
#define POINT_COUNT 3

static atomic_uint_least16_t MODULE_ENABLED = Model708Ena_Enabled;
static atomic_uint_least16_t ADOPT_CURVE_REQUEST = 0;

/// A (voltage, time) point on a synthetic curve, decreasing in voltage and increasing in trip
/// time as `pt_index`/`crv_index` grow - mirrors the tokio-modbus example's `curve_point`.
static void curve_point(uint16_t base_voltage_pct,
                         uint32_t base_time_tenths,
                         uint16_t crv_index,
                         uint16_t pt_index,
                         uint16_t *voltage,
                         uint32_t *time)
{
    *voltage = base_voltage_pct - crv_index * 5 - pt_index * 5;
    *time = base_time_tenths + (uint32_t)crv_index * 5 + (uint32_t)pt_index * 20;
}

Model708Ena der_trip_hv_module_enable_callback(const void *context)
{
    (void)context;
    return (Model708Ena)atomic_load(&MODULE_ENABLED);
}

void set_der_trip_hv_module_enable_callback(Model708Ena value, void *context)
{
    (void)context;
    atomic_store(&MODULE_ENABLED, (uint_least16_t)value);
}

uint16_t adopt_curve_request_callback(const void *context)
{
    (void)context;
    return atomic_load(&ADOPT_CURVE_REQUEST);
}

void set_adopt_curve_request_callback(uint16_t value, void *context)
{
    (void)context;
    atomic_store(&ADOPT_CURVE_REQUEST, value);
}

Model708AdptCrvRslt adopt_curve_result_callback(const void *context)
{
    (void)context;
    return Model708AdptCrvRslt_Completed;
}

uint16_t number_of_points_callback(const void *context)
{
    (void)context;
    return POINT_COUNT;
}

uint16_t stored_curve_count_callback(const void *context)
{
    (void)context;
    return CURVE_COUNT;
}

int16_t voltage_scale_factor_callback(const void *context)
{
    (void)context;
    return 0;
}

int16_t time_point_scale_factor_callback(const void *context)
{
    (void)context;
    return -1;
}

Model708ReadOnly crv_curve_access_callback(const void *context, uint16_t crv_index)
{
    (void)context;
    (void)crv_index;
    return Model708ReadOnly_Rw;
}

uint16_t must_trip_curve_crv_number_of_active_points_callback(const void *context, uint16_t crv_index)
{
    (void)context;
    (void)crv_index;
    return POINT_COUNT;
}

uint16_t may_trip_curve_crv_number_of_active_points_callback(const void *context, uint16_t crv_index)
{
    (void)context;
    (void)crv_index;
    return POINT_COUNT;
}

uint16_t momentary_cessation_curve_crv_number_of_active_points_callback(const void *context,
                                                                         uint16_t crv_index)
{
    (void)context;
    (void)crv_index;
    return POINT_COUNT;
}

uint16_t must_trip_curve_pt_voltage_point_callback(const void *context,
                                                     uint16_t crv_index,
                                                     uint16_t pt_index)
{
    (void)context;
    uint16_t voltage;
    uint32_t time;
    curve_point(120, 2, crv_index, pt_index, &voltage, &time);
    return voltage;
}

uint32_t must_trip_curve_pt_time_point_callback(const void *context,
                                                  uint16_t crv_index,
                                                  uint16_t pt_index)
{
    (void)context;
    uint16_t voltage;
    uint32_t time;
    curve_point(120, 2, crv_index, pt_index, &voltage, &time);
    return time;
}

uint16_t may_trip_curve_pt_voltage_point_callback(const void *context,
                                                    uint16_t crv_index,
                                                    uint16_t pt_index)
{
    (void)context;
    uint16_t voltage;
    uint32_t time;
    curve_point(115, 5, crv_index, pt_index, &voltage, &time);
    return voltage;
}

uint32_t may_trip_curve_pt_time_point_callback(const void *context,
                                                 uint16_t crv_index,
                                                 uint16_t pt_index)
{
    (void)context;
    uint16_t voltage;
    uint32_t time;
    curve_point(115, 5, crv_index, pt_index, &voltage, &time);
    return time;
}

uint16_t momentary_cessation_curve_pt_voltage_point_callback(const void *context,
                                                               uint16_t crv_index,
                                                               uint16_t pt_index)
{
    (void)context;
    uint16_t voltage;
    uint32_t time;
    curve_point(125, 1, crv_index, pt_index, &voltage, &time);
    return voltage;
}

uint32_t momentary_cessation_curve_pt_time_point_callback(const void *context,
                                                            uint16_t crv_index,
                                                            uint16_t pt_index)
{
    (void)context;
    uint16_t voltage;
    uint32_t time;
    curve_point(125, 1, crv_index, pt_index, &voltage, &time);
    return time;
}

void handle_panic(const char *message)
{
    fprintf(stderr, "Panic from rust lib: %s", message);
    exit(1);
}

/// Refreshes the inverter's per-phase voltages with new random values every
/// `VOLTAGE_REFRESH_INTERVAL_SECS`, for as long as the server runs.
void *randomise_voltages(void *arg)
{
    (void)arg;
    for (;;)
    {
        sleep(VOLTAGE_REFRESH_INTERVAL_SECS);
        atomic_store(&VOLTAGE_AN, (uint16_t)(2300 + rand() % 200));
        atomic_store(&VOLTAGE_BN, (uint16_t)(2300 + rand() % 200));
        atomic_store(&VOLTAGE_CN, (uint16_t)(2300 + rand() % 200));
    }
    return NULL;
}

int main(void)
{
    srand((unsigned int)time(NULL));

    modbus_t *ctx = modbus_new_tcp("127.0.0.1", 5502);
    if (!ctx)
        return 1;

    struct Model1CallbackAdapter common_adapter = {
        .context = NULL,
        .manufacturer_callback = manufacturer_callback,
        .model_callback = model_callback,
        .options_callback = options_callback,
        .version_callback = version_callback,
        .serial_number_callback = serial_number_callback,
        .device_address_callback = device_address_callback,
        .set_device_address_callback = set_device_address_callback,
    };

    struct Model103CallbackAdapter inverter_adapter = {
        .context = NULL,
        .amps_callback = amps_callback,
        .amps_phase_a_callback = amps_phase_a_callback,
        .amps_phase_b_callback = amps_phase_b_callback,
        .amps_phase_c_callback = amps_phase_c_callback,
        .a_sf_callback = a_sf_callback,
        .phase_voltage_an_callback = phase_voltage_an_callback,
        .phase_voltage_bn_callback = phase_voltage_bn_callback,
        .phase_voltage_cn_callback = phase_voltage_cn_callback,
        .v_sf_callback = v_sf_callback,
        .watts_callback = watts_callback,
        .w_sf_callback = w_sf_callback,
        .hz_callback = hz_callback,
        .hz_sf_callback = hz_sf_callback,
        .watt_hours_callback = watt_hours_callback,
        .wh_sf_callback = wh_sf_callback,
        .cabinet_temperature_callback = cabinet_temperature_callback,
        .tmp_sf_callback = tmp_sf_callback,
        .operating_state_callback = operating_state_callback,
        .event1_callback = event1_callback,
        .event_bitfield_2_callback = event_bitfield_2_callback,
    };

    struct Model708CallbackAdapter curve_adapter = {
        .context = NULL,
        .der_trip_hv_module_enable_callback = der_trip_hv_module_enable_callback,
        .set_der_trip_hv_module_enable_callback = set_der_trip_hv_module_enable_callback,
        .adopt_curve_request_callback = adopt_curve_request_callback,
        .set_adopt_curve_request_callback = set_adopt_curve_request_callback,
        .adopt_curve_result_callback = adopt_curve_result_callback,
        .number_of_points_callback = number_of_points_callback,
        .stored_curve_count_callback = stored_curve_count_callback,
        .voltage_scale_factor_callback = voltage_scale_factor_callback,
        .time_point_scale_factor_callback = time_point_scale_factor_callback,
        .crv_curve_access_callback = crv_curve_access_callback,
        .must_trip_curve_crv_number_of_active_points_callback =
            must_trip_curve_crv_number_of_active_points_callback,
        .may_trip_curve_crv_number_of_active_points_callback =
            may_trip_curve_crv_number_of_active_points_callback,
        .momentary_cessation_curve_crv_number_of_active_points_callback =
            momentary_cessation_curve_crv_number_of_active_points_callback,
        .must_trip_curve_pt_voltage_point_callback = must_trip_curve_pt_voltage_point_callback,
        .must_trip_curve_pt_time_point_callback = must_trip_curve_pt_time_point_callback,
        .may_trip_curve_pt_voltage_point_callback = may_trip_curve_pt_voltage_point_callback,
        .may_trip_curve_pt_time_point_callback = may_trip_curve_pt_time_point_callback,
        .momentary_cessation_curve_pt_voltage_point_callback =
            momentary_cessation_curve_pt_voltage_point_callback,
        .momentary_cessation_curve_pt_time_point_callback =
            momentary_cessation_curve_pt_time_point_callback,
    };

    static const CModelSpec model_list[] = {
        {.model_spec = &SUNSPEC_MODEL_1},
        {.model_spec = &SUNSPEC_MODEL_103},
        {.model_spec = &SUNSPEC_MODEL_708, .repeat_count_0 = CURVE_COUNT, .repeat_count_1 = POINT_COUNT},
    };
    size_t model_count = 3;

    // Read adapters cover every model, in map order.
    SunspecAdapter read_adapters[] = {
        sunspec_model_1_callback(&common_adapter),
        sunspec_model_103_callback(&inverter_adapter),
        sunspec_model_708_callback(&curve_adapter),
    };

    // Write adapters cover only the writable models, in map order. Model 103 has no writable
    // points, so it takes no entry here (its block still rejects writes).
    SunspecAdapter write_adapters[] = {
        sunspec_model_1_callback(&common_adapter),
        sunspec_model_708_callback(&curve_adapter),
    };
    size_t write_adapter_count = 2;

    pthread_t voltage_thread;
    if (pthread_create(&voltage_thread, NULL, randomise_voltages, NULL) != 0)
    {
        fprintf(stderr, "Failed to start voltage refresh thread\n");
        modbus_free(ctx);
        return 1;
    }
    pthread_detach(voltage_thread);

    int server_socket = modbus_tcp_listen(ctx, 1);
    if (server_socket == -1)
    {
        modbus_free(ctx);
        return 1;
    }

    uint8_t req[MODBUS_TCP_MAX_ADU_LENGTH];
    uint8_t res[MODBUS_TCP_MAX_ADU_LENGTH];
    while (modbus_tcp_accept(ctx, &server_socket) != -1)
    {
        for (;;)
        {
            int rc = modbus_receive(ctx, req);
            if (rc == -1)
            {
                if (errno == EMBBADCRC)
                    continue;
                printf("Broke - %d\n", errno);
                break; // client disconnected or fatal error
            }

            uint16_t tid = req[0] << 8 | req[1];
            uint8_t function_code = req[7];
            uint16_t address = req[8] << 8 | req[9];
            // Read/write-multiple: register count. Write-single: the value to write. Same
            // wire offset either way.
            uint16_t second_field = req[10] << 8 | req[11];

            printf("Function %d, addr %d, second field %d\n", function_code, address, second_field);

            if (function_code == MODBUS_FC_READ_HOLDING_REGISTERS)
            {
                uint16_t length = second_field;
                int bytes = length * 2;

                int32_t status = sunspec_read_registers(
                    model_list, model_count, address, &res[3], length, read_adapters, model_count);
                if (status == SUNSPEC_RC_OK)
                {
                    // Unit identifier
                    res[0] = 0;
                    // Function code
                    res[1] = function_code;
                    res[2] = (uint8_t)bytes;

                    modbus_send_raw_request_tid(ctx, res, bytes + 3, tid);
                    modbus_flush(ctx);
                }
                else if (status > 0)
                {
                    printf("Read rejected with Modbus exception %d\n", status);
                    modbus_reply_exception(ctx, req, (unsigned int)status);
                }
                else
                {
                    fprintf(stderr, "sunspec_service_read_registers failed: %d\n", status);
                    modbus_reply_exception(ctx, req, MODBUS_EXCEPTION_SLAVE_OR_SERVER_FAILURE);
                }
            }
            else if (function_code == MODBUS_FC_WRITE_MULTIPLE_REGISTERS)
            {
                uint16_t length = second_field;

                int32_t status = sunspec_write_multiple_registers(
                    model_list, model_count, address, &req[13], length, write_adapters, write_adapter_count);
                if (status == SUNSPEC_RC_OK)
                {
                    modbus_send_raw_request_tid(ctx, &req[6], 6, tid);
                    modbus_flush(ctx);
                }
                else if (status > 0)
                {
                    printf("Write rejected with Modbus exception %d\n", status);
                    modbus_reply_exception(ctx, req, (unsigned int)status);
                }
                else
                {
                    fprintf(stderr, "sunspec_service_write_registers failed: %d\n", status);
                    modbus_reply_exception(ctx, req, MODBUS_EXCEPTION_SLAVE_OR_SERVER_FAILURE);
                }
            }
            else if (function_code == MODBUS_FC_WRITE_SINGLE_REGISTER)
            {
                int32_t status = sunspec_write_single_register(
                    model_list, model_count, address, second_field, write_adapters, write_adapter_count);
                if (status == SUNSPEC_RC_OK)
                {
                    // FC 0x06's response echoes the request unchanged.
                    modbus_send_raw_request_tid(ctx, &req[6], 6, tid);
                    modbus_flush(ctx);
                }
                else if (status > 0)
                {
                    printf("Write rejected with Modbus exception %d\n", status);
                    modbus_reply_exception(ctx, req, (unsigned int)status);
                }
                else
                {
                    fprintf(stderr, "sunspec_service_write_registers failed: %d\n", status);
                    modbus_reply_exception(ctx, req, MODBUS_EXCEPTION_SLAVE_OR_SERVER_FAILURE);
                }
            }
            else
            {
                printf(
                    "SERVER: Exception::IllegalFunction - Unimplemented function code in request: %d\n",
                    function_code);
                modbus_reply_exception(ctx, req, MODBUS_EXCEPTION_ILLEGAL_FUNCTION);
            }
        }
    }

    modbus_close(ctx);
    modbus_free(ctx);
    return 0;
}
