#include <errno.h>
#include <modbus/modbus.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#include "../sunspec_modbus_codec.h"

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

St operating_state_callback(const void *context)
{
    (void)context;
    // model_103::St::Standby (discriminant 8). The header's generic `St` typedef is shared
    // with unrelated models' narrower variant sets, so there's no `St_Standby` constant to
    // name here.
    return 8;
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

    static const CModelSpec model_list[] = {
        {.model_spec = &SUNSPEC_MODEL_1},
        {.model_spec = &SUNSPEC_MODEL_103},
    };
    size_t model_count = 2;

    // Read adapters cover every model, in map order.
    SunspecAdapter read_adapters[] = {
        sunspec_model_1_callback(&common_adapter),
        sunspec_model_103_callback(&inverter_adapter),
    };

    // Write adapters cover only the writable models, in map order. Model 103 has no writable
    // points, so it takes no entry here (its block still rejects writes).
    SunspecAdapter write_adapters[] = {
        sunspec_model_1_callback(&common_adapter),
    };
    size_t write_adapter_count = 1;

    pthread_t voltage_thread;
    if (pthread_create(&voltage_thread, NULL, randomise_voltages, NULL) != 0)
    {
        fprintf(stderr, "Failed to start voltage refresh thread\n");
        modbus_free(ctx);
        return 1;
    }

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

                int32_t status = sunspec_service_read_registers(
                    model_list, model_count, read_adapters, model_count, address, length, &res[3]);
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

                int32_t status = sunspec_service_write_registers(
                    model_list, model_count, write_adapters, write_adapter_count, address, length, &req[13]);
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
                // Same layout as a one-register write via FC 0x10: address, then the value
                // itself, big-endian.
                uint8_t single_register[2] = {(uint8_t)(second_field >> 8), (uint8_t)(second_field & 0xFF)};

                int32_t status = sunspec_service_write_registers(
                    model_list, model_count, write_adapters, write_adapter_count, address, 1, single_register);
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
