#ifndef OBSBOT_WRAPPER_H
#define OBSBOT_WRAPPER_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef void* ObsbotDevicesCtx;
typedef void* ObsbotDeviceCtx;

ObsbotDevicesCtx obsbot_devices_get_instance();
int obsbot_devices_get_dev_num(ObsbotDevicesCtx ctx);
ObsbotDeviceCtx obsbot_devices_get_dev_by_index(ObsbotDevicesCtx ctx, int index);
ObsbotDeviceCtx obsbot_devices_get_dev_by_sn(ObsbotDevicesCtx ctx, const char* sn);

int obsbot_dev_get_sn(ObsbotDeviceCtx dev, char* buffer, int max_len);
int obsbot_dev_get_version(ObsbotDeviceCtx dev, char* buffer, int max_len);
int obsbot_dev_get_model(ObsbotDeviceCtx dev, char* buffer, int max_len);
bool obsbot_dev_is_connected(ObsbotDeviceCtx dev);

int obsbot_meet_get_media_mode(ObsbotDeviceCtx dev);
void obsbot_meet_set_media_mode(ObsbotDeviceCtx dev, int mode);

int obsbot_meet_get_auto_framing_type(ObsbotDeviceCtx dev);
void obsbot_meet_set_auto_framing_type(ObsbotDeviceCtx dev, int type);

int obsbot_meet_get_hdr(ObsbotDeviceCtx dev);
void obsbot_meet_set_hdr(ObsbotDeviceCtx dev, int enable);

void obsbot_image_set_brightness(ObsbotDeviceCtx dev, int val);
void obsbot_image_set_contrast(ObsbotDeviceCtx dev, int val);
void obsbot_image_set_saturation(ObsbotDeviceCtx dev, int val);
void obsbot_image_set_hue(ObsbotDeviceCtx dev, int val);
void obsbot_image_set_sharpness(ObsbotDeviceCtx dev, int val);
void obsbot_image_set_white_balance(ObsbotDeviceCtx dev, int auto_, int temp);

void obsbot_camera_set_zoom(ObsbotDeviceCtx dev, float zoom);

void obsbot_camera_set_focus(ObsbotDeviceCtx dev, int auto_, int val);

void obsbot_camera_set_anti_flicker(ObsbotDeviceCtx dev, int val);

void obsbot_camera_set_background_blur(ObsbotDeviceCtx dev, int level);

void obsbot_camera_reset_default(ObsbotDeviceCtx dev);

bool obsbot_dev_is_inited(ObsbotDeviceCtx dev);

#ifdef __cplusplus
}
#endif

#endif
