var SystemInfo = {
  icon: {
    value: null,
    asKey: function () {
      return "icon";
    },
  },
  systemName: {
    value: "parrot",
    asKey: function () {
      return "systemName";
    },
  },

  host: {
    osName: {
      value: "Release 6.0 (lorikeet) 64-bit",
      asKey: function () {
        return "osName";
      },
    },
    kernel: {
      value: "Kernel Linux 6.5.0-13parrot1-amd64 x86_64",
      asKey: function () {
        return "kernel";
      },
    },
    desktop: {
      value: "MATE 1.26.0",
      asKey: function () {
        return "desktop";
      },
    },

    asKey: function () {
      return "host";
    },
  },

  hardware: {
    memory: {
      value: 15 * 1024 * 1024,
      asKey: function () {
        return "memory";
      },
    }, // in KiB
    processor: {
      value: "AMD RYzen 7 5800H with Radeon Graphics x 16",
      asKey: function () {
        return "processor";
      },
    },
    graphics: {
      value:
        "AMD Radeon Graphics (renoir, LLVM 15.0.6, DRM 3.54, 6.5.0-13parrot1-amd64)",
      asKey: function () {
        return "graphics";
      },
    },

    asKey: function () {
      return "hardware";
    },
  },

  systemStatus: {
    diskSpace: {
      value: (30.3 * 1024 * 1024) + " out of " + (53.7 * 1024 * 1024 ),
      asKey: function () {
        return "availableDiskSpace";
      },
    },

    asKey: function () {
      return "systemStatus";
    },
  },
};

let DomSystemInfo = new Map();

function InitilizeTmSystemInfo() {
  let setDom = function (parentPath = "", value) {
    let key = value.asKey();
    if (parentPath.length > 0) {
      key = parentPath + "." + key;
    }

    let targetElement = document.getElementById("systemInfo." + key);
    if (targetElement == null) {
      console.log("[InitilizeTmSystemInfo] targetElement is null. key: " + key);
    }

    DomSystemInfo[key] = targetElement;
  };

  let parentPath = "";
  setDom(parentPath, SystemInfo.icon);
  setDom(parentPath, SystemInfo.systemName);

  parentPath = SystemInfo.host.asKey();
  setDom(parentPath, SystemInfo.host.desktop);
  setDom(parentPath, SystemInfo.host.kernel);
  setDom(parentPath, SystemInfo.host.osName);

  parentPath = SystemInfo.hardware.asKey();
  setDom(parentPath, SystemInfo.hardware.graphics);
  setDom(parentPath, SystemInfo.hardware.memory);
  setDom(parentPath, SystemInfo.hardware.processor);

  parentPath = SystemInfo.systemStatus.asKey();
  setDom(parentPath, SystemInfo.systemStatus.diskSpace);

  updateSystemInfo();
}

function updateSystemInfo() {
  // when encountered error
  let onError = function (error) {
    console.log(`Call to SystemInfo failed with error: ${error}`);
  };

  // When got success
  // this only means the network call was ok,
  // we still needs to check if the api has returned ok or error
  let onSuccess = function (systemSpecsRes) {
    okSystemSpecs = systemSpecsRes.result;
    if (okSystemSpecs == null && systemSpecsRes.error == null) {
      console.log(
        "[updateSystemInfo] systemSepcs does not contain .result nor .error"
      );
    } else if (okSystemSpecs == null && okSystemSpecs.error != null) {
      console.log(
        `[updateSystemInfo] systemSpecs is null. systemSpecsRes: ${JSON.stringify(
          systemSpecsRes
        )}`
      );
    }
    if (okSystemSpecs != null && okSystemSpecs.error == null) {
      console.log("Got systemSpecs: ");
      console.log(okSystemSpecs);

      // update DomSystemInfo
      DomSystemInfo[SystemInfo.icon.asKey()].src = okSystemSpecs.icon;
      DomSystemInfo[SystemInfo.systemName.asKey()].innerText =
        okSystemSpecs.systemName;

      let prefix = SystemInfo.host.asKey() + ".";

      DomSystemInfo[prefix + SystemInfo.host.desktop.asKey()].innerText =
        okSystemSpecs.desktop;

      DomSystemInfo[prefix + SystemInfo.host.kernel.asKey()].innerText =
        okSystemSpecs.kernelVersion;

      DomSystemInfo[prefix + SystemInfo.host.osName.asKey()].innerText =
        okSystemSpecs.longOsVersion;

      prefix = SystemInfo.hardware.asKey() + ".";

      DomSystemInfo[prefix + SystemInfo.hardware.memory.asKey()].innerText =
        (okSystemSpecs.totalMemory / 1024 / 1024 / 1024).toFixed(2) + " GiB";

      DomSystemInfo[prefix + SystemInfo.hardware.graphics.asKey()].innerText =
        okSystemSpecs.graphics;

      let cpus = new Map();
      okSystemSpecs.cpusInfo.forEach(function (cpuInfo) {
        cpus[cpuInfo.brand] =
          cpus[cpuInfo.brand] == null ? 1 : cpus[cpuInfo.brand] + 1;
      });
      let cpuString = "";
      for (const [key, value] of Object.entries(cpus)) {
        cpuString += key + "x " + value;
      }
      DomSystemInfo[prefix + SystemInfo.hardware.processor.asKey()].innerText =
        cpuString + " | " + okSystemSpecs.cpuArch;

      prefix = SystemInfo.systemStatus.asKey() + ".";

      let totalSpace = 0;
      let availableSpace = 0;
      okSystemSpecs.disksInfo.forEach(function (diskInfo) {
        totalSpace += diskInfo.totalSpace;;
        availableSpace += diskInfo.availableSpace;
      });
      DomSystemInfo[
        prefix + SystemInfo.systemStatus.diskSpace.asKey()
      ].innerText = ( availableSpace / 1024/ 1024 / 1024).toFixed(2) + " GiB out of " + (totalSpace / 1024 / 1024 / 1024).toFixed(2) + " GiB";
    }
  };

  // make call
  getSystemSpecs(onSuccess, onError);
}
