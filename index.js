import { extractAll, createPackageWithOptions } from "@electron/asar";
import { readdirSync, readFileSync, writeFile, existsSync } from "fs";
import rimraf from "rimraf";
import { dirname, normalize } from "path";
import { spawn } from "child_process";
import { exit } from "process";

function patchFile(path) {
  let content = readFileSync(path).toString();

  const adsense_patch = "https://gist.githubusercontent.com/MidKnightXI/7ecf3cdd0a5804466cb790855e2524ae/raw/9b88cf64f3bb955edfff27bdfba72f5181d8748b/remover.txt";
  const AMERICA = '["US","CA"].includes';
  const EU = '["AD","AL","AT","AX","BA","BE","BG","BY","CH","CY","CZ","DE","DK","EE","ES","FI","FO","FR","GB","GG","GI","GR","HR","HU","IE","IM","IS","IT","JE","LI","LT","LU","LV","MC","MD","ME","MK","MT","NL","NO","PL","PT","RO","RS","RU","SE","SI","SJ","SK","SM","UA","VA","XK"].includes';

  content = content.replaceAll(
    "https://dtapp-player.op.gg/adsense.txt",
    adsense_patch
  );
  content = content.replace(
    /exports\.countryHasAds=\w;/gm,
    "exports.countryHasAds=[];"
  );
  content = content.replace(
    /exports\.countryHasAdsAdsense=\w;/gm,
    "exports.countryHasAdsAdsense=[];"
  );
  content = content.replace(
    /exports\.adsenseAds=\w;/gm,
    "exports.adsenseAds=[];"
  );
  content = content.replace(
    /exports\.playwireAds=\w;/gm,
    "exports.playwireAds=[];"
  );
  content = content.replace(
    /exports\.nitropayAds=\w;/gm,
    "exports.nitropayAds=[];"
  );

  content = content.replaceAll(
    "google-analytics.com/mp/collect",
    "gist.githubusercontent.com"
  );

  content = content.replaceAll(AMERICA, "[].includes");
  content = content.replaceAll(EU, "[].includes");

  writeFile(path, content, () => console.log(`PatchFile: rewriting ${path}`));
  return;
}

async function scanDir(asarFilePath) {
  console.log("Unpacking OPGG asar file");
  extractAll(asarFilePath, "op-gg-unpacked");

  const assetDir = normalize("op-gg-unpacked/assets/react");
  const assetFiles = readdirSync(assetDir);

  for (let fileName of assetFiles) {
    if (fileName.endsWith(".js")) {
      patchFile(normalize(`${assetDir}/${fileName}`));
    }
  }

  console.log(`Rebuilding ${asarFilePath} without ads urls`);
  await createPackageWithOptions("op-gg-unpacked", asarFilePath, {
    unpackDir: "{node_modules/node-ovhook,node_modules/rust-process}",
  });

  rimraf("op-gg-unpacked", () => {
    console.log(`scanDir: deleted temporary directory`)
  });
  return;
}

function killProcess() {
  console.log("killProcess: killing OPGG process");

  if (process.platform === "darwin") {
    spawn("killall", ["-9", "OP.GG"]);
  }
  else if (process.platform === "win32") {
    spawn("taskkill", ["/im", "OP.GG.exe", "/F"]);
  }
  return;
}

function asarDir() {
  let directory = null;

  if (process.platform === "darwin") {
    directory =  normalize("/Applications/OP.GG.app/Contents/Resources/app.asar");
  } else if (process.platform === "win32") {
    directory = normalize(
      `${dirname(process.env.APPDATA)}/Local/Programs/OP.GG/resources/app.asar`);
  }
  else {
    console.error("asarDir: platform not supported.")
    exit(0);
  }
  return directory;
}

function main() {
  const asarFilePath = asarDir();

  if (!existsSync(asarFilePath)) {
    console.error(`Cannot find asar file at ${asarFilePath}`);
    return 84;
  }
  killProcess();
  scanDir(asarFilePath);
  return;
}

main();
