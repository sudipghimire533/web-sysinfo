let systemRootApi = "http://127.0.0.1:8080/v1/sysinfo";

var okSystemSpecs = {
  totalMemory: 0,
  cpuArch: "unknown",
  cpuName: "unknown",
  cpuBrand: "unknown",
  cpuVendorId: 0,
  distributionId: "unknown",
  kernelVersion: "unknown",
  osRelease: "unknown",
  physicalCoreCount: 0,
  systemName: "unknown",
  totalSwap: 0,
  cpusInfo: [
    {
      brand: "unknown",
      name: "unknown",
      vendor_id: "unknown",
    },
  ],
  disksInfo: [
    {
      availableSpace: 0,
      totalSpace: 0,
      filesystem: "unknown",
      name: "unknown",
    }
  ],
};

var systemSpecsResponse = {
  result: okSystemSpecs,
  error: null,
};

function getSystemSpecs(onSuccess, onFailure) {
  let apiPath = systemRootApi + "/system-specs/get-system-specs";
  axios
    .get(apiPath)
    .then(function (response) {
      onSuccess(JSON.parse(response.data));
    })

    .catch(function (error) {
      console.log(`Call: ${apiPath} failed with: ${error}`);
      onFailure(error);
    })

    .finally(function () {});
}
