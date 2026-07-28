#ifndef SUNSPEC_MODBUS_CODEC_H
#define SUNSPEC_MODBUS_CODEC_H

#pragma once

/* Generated with cbindgen:0.29.4 */

typedef struct Model1Adapter {
  const char *(*manufacturer_callback)(void);
  const char *(*model_callback)(void);
  const char *(*options_callback)(void);
  const char *(*version_callback)(void);
  const char *(*serial_number_callback)(void);
  uint16_t (*device_address_callback)(void);
  void (*set_device_address_callback)(uint16_t value);
} Model1Adapter;

typedef struct SunspecModelAdapters {
  const struct Model1Adapter *model_1_adapter;
} SunspecModelAdapters;

typedef struct SunspecService {
  struct SunspecModelAdapters adapters;
} SunspecService;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern int32_t printf(const char *format, ...);

extern int32_t printf(const char *format, ...);

extern int32_t printf(const char *format, ...);

extern int32_t printf(const char *format, ...);

int32_t sunspec_service_handle_request(const struct SunspecService *service,
                                       uint16_t address,
                                       uint16_t length,
                                       uint16_t *response_buffer);

void sunspec_service_init(struct SunspecService *service, const struct Model1Adapter *adapter);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* SUNSPEC_MODBUS_CODEC_H */
