#include <stdio.h>

#include <modbus/modbus.h>
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
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
    struct SunspecService sunspec_service;
    sunspec_service_init(&sunspec_service, &sunspec_common_adapter);

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

                    sunspec_service_handle_request(&sunspec_service, address, length, (uint16_t *)&res[3]);

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