const path = require("path");
const programDir = path.join(__dirname, "programs/mtoken");
const idlDir = path.join(__dirname, "sdk", "idl");
const sdkDir = path.join(__dirname, "sdk", "generated");
const binaryInstallDir = path.join(__dirname, "..", "..", "target", "solita");

module.exports = {
  idlGenerator: "anchor",
  programName: "mtoken",
  programId: "mtokYxNhZEihbDq3r6LX22pLKnpuQvXV5kwhgCDCWw4",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
