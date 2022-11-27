const path = require("path");
const programDir = path.join(__dirname, "..", "programs", "open_creator_protocol");
const idlDir = path.join(__dirname, "idl");
const sdkDir = path.join(__dirname, "src", "generated");
const binaryInstallDir = path.join(__dirname, "..", "target", "solita");

module.exports = {
  idlGenerator: "anchor",
  programName: "open_creator_protocol",
  programId: "ocp4vWUzA2z2XMYJ3QhM9vWdyoyoQwAFJhRdVTbvo9E",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
