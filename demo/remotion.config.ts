import { Config } from "@remotion/cli/config";

Config.setVideoImageFormat("jpeg");
Config.setOverwriteOutput(true);
// Render at 2x for crisp text, then downscale when converting to GIF.
Config.setConcurrency(4);
