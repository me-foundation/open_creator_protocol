import type { SendTransactionError } from "@solana/web3.js";
import { default as colors } from "colors/safe.js";
import {
  InstructionLogs,
  LogMessage,
  parseProgramLogs,
} from "./parseTransactionLogs";

/**
 * Formats a log entry to be printed out.
 * @param entry
 * @param prefix
 * @returns
 */
export const formatLogEntry = (entry: LogMessage): string => {
  switch (entry.style) {
    case "success":
      return `Program returned success`;
    case "muted":
      return `Program returned error: ${entry.text}`;
    case "info":
      return `Runtime error: ${entry.text}`;
    case "warning":
      return entry.text;
  }
};

/**
 * Formats instruction logs to be printed to the console.
 * @param logs
 */
export const formatInstructionLogsForConsole = (
  logs: readonly InstructionLogs[]
): string =>
  logs
    .map((log, i) => {
      return [
        [
          colors.bold(colors.blue("=> ")),
          colors.bold(colors.white(`Instruction #${i}: `)),
          log.invokedProgram
            ? colors.yellow(`Program ${log.invokedProgram}`)
            : "System",
        ].join(""),
        ...log.logs.map((entry) => {
          const entryStr = formatLogEntry(entry);
          switch (entry.style) {
            case "info":
              return colors.white(entryStr);
            case "warning":
              return colors.cyan(entryStr);
            case "muted":
              return colors.white(entryStr);
            case "success":
              return colors.green(entryStr);
          }
        }),
      ].join("\n");
    })
    .join("\n");

export const printSendTransactionError = (err: SendTransactionError) => {
  try {
    const parsed = parseProgramLogs(err.logs ?? [], err);
    console.log(formatInstructionLogsForConsole(parsed));
  } catch (e) {
    console.warn(
      colors.yellow("Could not print logs due to error. Printing raw logs"),
      e
    );
    console.log(err.logs?.join("\n"));
  }
};
