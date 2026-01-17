#include "obsbot_wrapper.h"
#include "dev/devs.hpp"
#include "dev/dev.hpp"
#include <string.h>
#include <vector>
#include <iostream>

int copy_string(const std::string& src, char* buffer, int max_len) {
    if (!buffer || max_len <= 0) return -1;
    strncpy(buffer, src.c_str(), max_len - 1);
    buffer[max_len - 1] = '\0';
    return strlen(buffer);
}

ObsbotDevicesCtx obsbot_devices_get_instance() {
    return &Devices::get();
}

int obsbot_devices_get_dev_num(ObsbotDevicesCtx ctx) {
    Devices* devices = static_cast<Devices*>(ctx);
    return (int)devices->getDevNum();
}

ObsbotDeviceCtx obsbot_devices_get_dev_by_index(ObsbotDevicesCtx ctx, int index) {
    Devices* devices = static_cast<Devices*>(ctx);
    auto devList = devices->getDevList();
    
    if (index < 0 || index >= devList.size()) return nullptr;
    
    auto it = devList.begin();
    std::advance(it, index);
    return it->get(); 
}

ObsbotDeviceCtx obsbot_devices_get_dev_by_sn(ObsbotDevicesCtx ctx, const char* sn) {
    if (!sn) return nullptr;
    Devices* devices = static_cast<Devices*>(ctx);
    // Since getDevBySn takes std::string, we convert
    std::string sn_str(sn);
    auto dev = devices->getDevBySn(sn_str);
    return dev ? dev.get() : nullptr;
}

int obsbot_dev_get_sn(ObsbotDeviceCtx dev, char* buffer, int max_len) {
    Device* device = static_cast<Device*>(dev);
    return copy_string(device->devSn(), buffer, max_len);
}

int obsbot_dev_get_version(ObsbotDeviceCtx dev, char* buffer, int max_len) {
    Device* device = static_cast<Device*>(dev);
    return copy_string(device->devVersion(), buffer, max_len);
}

int obsbot_dev_get_model(ObsbotDeviceCtx dev, char* buffer, int max_len) {
    Device* device = static_cast<Device*>(dev);
    return copy_string(device->devModelCode(), buffer, max_len);
}

bool obsbot_dev_is_inited(ObsbotDeviceCtx dev) {
    Device* device = static_cast<Device*>(dev);
    return device->isInited();
}

bool obsbot_dev_is_connected(ObsbotDeviceCtx dev) {
     Device* device = static_cast<Device*>(dev);
     return device->isInited();
}

int obsbot_meet_get_media_mode(ObsbotDeviceCtx dev) {
    Device* device = static_cast<Device*>(dev);
    Device::CameraStatus status = device->cameraStatus();
    return (int)status.meet.media_mode;
}

void obsbot_meet_set_media_mode(ObsbotDeviceCtx dev, int mode) {
    Device* device = static_cast<Device*>(dev);
    device->cameraSetMediaModeU(static_cast<Device::MediaMode>(mode));
}

int obsbot_meet_get_auto_framing_type(ObsbotDeviceCtx dev) {
    Device* device = static_cast<Device*>(dev);
    Device::CameraStatus status = device->cameraStatus();
    return (int)status.meet.group_single;
}

void obsbot_meet_set_auto_framing_type(ObsbotDeviceCtx dev, int type) {
    Device* device = static_cast<Device*>(dev);
    device->cameraSetAutoFramingModeU(
        static_cast<Device::AutoFramingType>(type), 
        Device::AutoFrmCloseUp 
    );
}

int obsbot_meet_get_hdr(ObsbotDeviceCtx dev) {
    Device* device = static_cast<Device*>(dev);
    Device::CameraStatus status = device->cameraStatus();
    return (int)status.meet.hdr;
}

void obsbot_meet_set_hdr(ObsbotDeviceCtx dev, int enable) {
    Device* device = static_cast<Device*>(dev);
    // dev.hpp says cameraSetWdrR is for meet series.
    // However, libdev.so does NOT contain this symbol. 
    // Commenting out to fix link error.
    // device->cameraSetWdrR(enable ? Device::DevWdrModeDol2TO1 : Device::DevWdrModeNone);
    printf("Warning: cameraSetWdrR symbol missing in SDK lib. HDR setting ignored.\n");
}

// Image Controls
void obsbot_image_set_brightness(ObsbotDeviceCtx dev, int val) {
    static_cast<Device*>(dev)->cameraSetImageBrightnessR(val);
}

void obsbot_image_set_contrast(ObsbotDeviceCtx dev, int val) {
    static_cast<Device*>(dev)->cameraSetImageContrastR(val);
}

void obsbot_image_set_saturation(ObsbotDeviceCtx dev, int val) {
    static_cast<Device*>(dev)->cameraSetImageSaturationR(val);
}

void obsbot_image_set_hue(ObsbotDeviceCtx dev, int val) {
    static_cast<Device*>(dev)->cameraSetImageHueR(val);
}

void obsbot_image_set_sharpness(ObsbotDeviceCtx dev, int val) {
    static_cast<Device*>(dev)->cameraSetImageSharpR(val);
}

void obsbot_image_set_white_balance(ObsbotDeviceCtx dev, int auto_, int temp) {
    if (auto_) {
        static_cast<Device*>(dev)->cameraSetWhiteBalanceR(Device::DevWhiteBalanceAuto, 0);
    } else {
        static_cast<Device*>(dev)->cameraSetWhiteBalanceR(Device::DevWhiteBalanceManual, temp);
    }
}

void obsbot_camera_set_zoom(ObsbotDeviceCtx dev, float zoom) {
    static_cast<Device*>(dev)->cameraSetZoomAbsoluteR(zoom);
}

void obsbot_camera_set_focus(ObsbotDeviceCtx dev, int auto_, int val) {
    Device* device = static_cast<Device*>(dev);
    if (auto_) {
        device->cameraSetAutoFocusModeR(Device::DevAutoFocusAFC);
    } else {
        device->cameraSetFocusAbsolute(val, false);
    }
}

void obsbot_camera_set_anti_flicker(ObsbotDeviceCtx dev, int val) {
    // 0=Off, 1=50, 2=60, 3=Auto
    static_cast<Device*>(dev)->cameraSetAntiFlickR(val);
}

void obsbot_camera_set_background_blur(ObsbotDeviceCtx dev, int level) {
    // Requires MediaMode to be Background and BgMode to be Blur?
    // SDK says: "MediaMode should be set to MediaModeBackground... and MediaBgMode should be set to MediaBgModeBlur"
    // For now we just call setMaskLevelU. User might need to set mode separately.
    static_cast<Device*>(dev)->cameraSetMaskLevelU(level);
}

void obsbot_camera_reset_default(ObsbotDeviceCtx dev) {
    static_cast<Device*>(dev)->cameraSetRestoreFactorySettingsR();
}


