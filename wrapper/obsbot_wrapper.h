#ifndef OBSBOT_WRAPPER_H
#define OBSBOT_WRAPPER_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handles
typedef void* ObsbotDevicesCtx;
typedef void* ObsbotDeviceCtx;

// Devices Management
ObsbotDevicesCtx obsbot_devices_get_instance();
int obsbot_devices_get_dev_num(ObsbotDevicesCtx ctx);
ObsbotDeviceCtx obsbot_devices_get_dev_by_index(ObsbotDevicesCtx ctx, int index); // Not efficient but simple for CLI
ObsbotDeviceCtx obsbot_devices_get_dev_by_sn(ObsbotDevicesCtx ctx, const char* sn);

// Device Info
// Returns length of string written, or -1 on error
int obsbot_dev_get_sn(ObsbotDeviceCtx dev, char* buffer, int max_len);
int obsbot_dev_get_version(ObsbotDeviceCtx dev, char* buffer, int max_len);
int obsbot_dev_get_model(ObsbotDeviceCtx dev, char* buffer, int max_len);
bool obsbot_dev_is_connected(ObsbotDeviceCtx dev);

// AI / Feature Controls (Meet SE specific mappings)
// 0: Normal, 1: Background, 2: AutoFrame
int obsbot_meet_get_media_mode(ObsbotDeviceCtx dev);
void obsbot_meet_set_media_mode(ObsbotDeviceCtx dev, int mode);

// 0: Group, 1: Single
int obsbot_meet_get_auto_framing_type(ObsbotDeviceCtx dev);
void obsbot_meet_set_auto_framing_type(ObsbotDeviceCtx dev, int type);

// HDR: 0 off, 1 on
int obsbot_meet_get_hdr(ObsbotDeviceCtx dev);
void obsbot_meet_set_hdr(ObsbotDeviceCtx dev, int enable);

// Webcam State
bool obsbot_dev_is_inited(ObsbotDeviceCtx dev);

#ifdef __cplusplus
}
#endif

#endif
