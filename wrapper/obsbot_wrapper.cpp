#include "obsbot_wrapper.h"
#include "dev/devs.hpp"
#include "dev/dev.hpp"
#include <string.h>
#include <vector>
#include <iostream>

// Helper to copy std::string to C buffer
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
     // Checking connection state usually involves checking if it's still in the device list or responding.
     // For now, we assume if we have a handle it might be valid, but isInited is a decent proxy for "ready".
     return device->isInited();
}

// MEET SE Specifics

int obsbot_meet_get_media_mode(ObsbotDeviceCtx dev) {
    Device* device = static_cast<Device*>(dev);
    Device::CameraStatus status = device->cameraStatus();
    // Assuming Meet structure is active for Meet SE
    return (int)status.meet.media_mode;
}

void obsbot_meet_set_media_mode(ObsbotDeviceCtx dev, int mode) {
    Device* device = static_cast<Device*>(dev);
    device->cameraSetMediaModeU(static_cast<Device::MediaMode>(mode));
}

int obsbot_meet_get_auto_framing_type(ObsbotDeviceCtx dev) {
    Device* device = static_cast<Device*>(dev);
    Device::CameraStatus status = device->cameraStatus();
    // Return group_single value (0: Group, 1: Single)
    return (int)status.meet.group_single;
}

void obsbot_meet_set_auto_framing_type(ObsbotDeviceCtx dev, int type) {
    Device* device = static_cast<Device*>(dev);
    // If type is 0 (Group), we set AutoFrmGroup.
    // If type is 1 (Single), we set AutoFrmSingle.
    // We ignore the second parameter (close_upper) for this simple wrapper or default it.
    device->cameraSetAutoFramingModeU(
        static_cast<Device::AutoFramingType>(type), 
        Device::AutoFrmCloseUp // Default
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
    // DevWdrModeNone = 0, DevWdrModeDol2TO1 = 1 (HDR enabled)
    device->cameraSetWdrR(enable ? Device::DevWdrModeDol2TO1 : Device::DevWdrModeNone);
}
