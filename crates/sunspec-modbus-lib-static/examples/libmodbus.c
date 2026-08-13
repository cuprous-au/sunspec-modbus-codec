#include <errno.h>
#include <modbus/modbus.h>
#include <stdio.h>
#include <stdlib.h>

#include "../sunspec_modbus_codec.h"

const char *manufacturer_callback()
{
    static char static_content[32] = "Cuprous";
    return static_content;
};

const char *model_callback()
{
    static char static_content[32] = "My Model";
    return static_content;
};

const char *serial_number_callback()
{
    static char static_content2[32] = "ABC-123";
    printf("Getting serial number\n");
    return static_content2;
};

uint16_t zero_callback_u16()
{
    printf("zero callback - u16");
    return 0;
};

uint32_t zero_callback_u32()
{
    return 0;
};

int16_t zero_callback_i16()
{
    return 0;
};

void noop_callback() {
};

void handle_panic(const char* message) {
    fprintf(stderr, "Panic from rust lib: %s", message);
    exit(1);
};

typedef struct BatteryState
{
    uint32_t cycle_count;
    uint16_t alarm_reset;
    SetOp operation;
    SetInvState inverter_state;
} BatteryState;

uint32_t cycle_count_callback(const void *self)
{
    return (*(struct BatteryState *)self).cycle_count;
}

uint16_t alarm_reset_callback(const void *self)
{
    return (*(struct BatteryState *)self).alarm_reset;
}

SetOp operation_callback(const void *self)
{
    return (*(struct BatteryState *)self).operation;
}

SetInvState inverter_state_callback(const void *self)
{
    return (*(struct BatteryState *)self).inverter_state;
}

void set_alarm_reset_callback(uint16_t value, void *self)
{
    (*(struct BatteryState *)self).alarm_reset = value;
}

void set_operation_callback(SetOp value, void *self)
{
    (*(struct BatteryState *)self).operation = value;
}

void set_inverter_state_callback(SetInvState value, void *self)
{
    (*(struct BatteryState *)self).inverter_state = value;
}

int main(void)
{
    modbus_t *ctx = modbus_new_tcp("127.0.0.1", 5502);
    if (!ctx)
        return 1;

    // 0 coils, 0 discrete inputs, 100 holding regs, 0 input regs
    modbus_mapping_t *map = modbus_mapping_new(0, 0, 100, 0);
    if (!map)
    {
        modbus_free(ctx);
        return 1;
    }
    struct Model1CallbackAdapter sunspec_common_adapter = {
        .manufacturer_callback = manufacturer_callback,
        .model_callback = model_callback,
        .serial_number_callback = serial_number_callback,
    };

    struct BatteryState battery_state = {
        .cycle_count = 10,
        .alarm_reset = 0,
        .operation = SetOp_Disconnect,
        .inverter_state = SetInvState_InverterStandby,
    };

    struct Model802CallbackAdapter sunspec_battery_adapter =
        {
            .context = &battery_state,
            .nameplate_charge_capacity_callback = zero_callback_u16,
            .nameplate_energy_capacity_callback = zero_callback_u16,
            .nameplate_max_charge_rate_callback = zero_callback_u16,
            .nameplate_max_discharge_rate_callback = zero_callback_u16,
            .state_of_charge_callback = zero_callback_u16,
            .cycle_count_callback = cycle_count_callback,
            .control_mode_callback = zero_callback_u16,
            .alarm_reset_callback = alarm_reset_callback,
            .battery_type_callback = zero_callback_u16,
            .state_of_the_battery_bank_callback = zero_callback_u16,
            .battery_event_1_bitfield_callback = zero_callback_u32,
            .battery_event_2_bitfield_callback = zero_callback_u32,
            .vendor_event_bitfield_1_callback = zero_callback_u32,
            .vendor_event_bitfield_2_callback = zero_callback_u32,
            .external_battery_voltage_callback = zero_callback_u16,
            .total_dc_current_callback = zero_callback_i16,
            .total_power_callback = zero_callback_i16,
            .ah_rtg_sf_callback = zero_callback_u16,
            .wh_rtg_sf_callback = zero_callback_u16,
            .w_cha_dis_cha_max_sf_callback = zero_callback_u16,
            .so_c_sf_callback = zero_callback_u16,
            .v_sf_callback = zero_callback_u16,
            .cell_v_sf_callback = zero_callback_u16,
            .a_sf_callback = zero_callback_u16,
            .a_max_sf_callback = zero_callback_u16,
            .inverter_state_callback = inverter_state_callback,
            .operation_callback = operation_callback,
            .set_alarm_reset_callback = set_alarm_reset_callback,
            .set_operation_callback = set_operation_callback,
            .set_inverter_state_callback = set_inverter_state_callback,
        };

    struct Model103StatefulAdapter inverter_adapter =
        {
            .amps = 1,
            .amps_phase_a = 2,
            .amps_phase_b = 3,
            .amps_phase_c = 4,
            .a_sf = 0,
            .phase_voltage_ab = 0,
            .phase_voltage_bc = 0,
            .phase_voltage_ca = 0,
            .phase_voltage_an = 0,
            .phase_voltage_bn = 0,
            .phase_voltage_cn = 0,
        };

    struct SunspecExternalAdapters adapters =
        {
            .model_1_callback_adapter = &sunspec_common_adapter,
            .model_802_callback_adapter = &sunspec_battery_adapter,
            .model_103_stateful_adapter = &inverter_adapter,
        };

    int server_socket = modbus_tcp_listen(ctx, 1);
    if (server_socket == -1)
        goto cleanup;

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
            else
            {
                uint16_t tid = req[0] << 8 | req[1];
                uint8_t device_id = req[6];
                uint8_t function_code = req[7];
                uint16_t address = req[8] << 8 | req[9];
                uint16_t length = req[10] << 8 | req[11];

                printf("Function %d, addr %d, len %d\n", function_code, address, length);
                if (function_code == 3)
                {
                    // Only holding register reads are supported at this point;

                    // Reuse request buffer, copying values from initial request
                    int bytes = (length) * 2;

                    // Unit identifier
                    res[0] = 0;

                    // Function code
                    res[1] = function_code;
                    res[2] = bytes;

                    sunspec_service_handle_request(&adapters, address, length, &res[3]);

                    printf("Responding with %d bytes\n", bytes + 3);
                    for (int i = 0; i < bytes + 3; i++)
                    {
                        printf("%02x ", res[i]);
                    }
                    printf("\n");

                    modbus_send_raw_request_tid(ctx, res, bytes + 3, tid);
                    modbus_flush(ctx);
                }
                else
                {
                    if (modbus_reply(ctx, req, rc, map) == -1)
                    {
                        break;
                    }
                }
            }
        }
    }

cleanup:
    modbus_mapping_free(map);
    modbus_close(ctx);
    modbus_free(ctx);
    return 0;
}
