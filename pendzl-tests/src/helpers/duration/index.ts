import { millis } from "./millis";
import { seconds } from "./seconds";
import { minutes } from "./minutes";
import { hours } from "./hours";
import { days } from "./days";
import { weeks } from "./weeks";
import { years } from "./years";

export default {
  milis: millis,
  seconds: seconds,
  minutes: minutes,
  hours: hours,
  days: days,
  weeks: weeks,
  years: years,
} as const;
