import { extractAll, createPackageWithOptions } from "asar";
import { readdirSync, readFileSync, writeFileSync, existsSync } from "fs";
import { sync } from "rimraf";
import { dirname, normalize } from "path";
import { spawn } from "child_process";

function replaceAdFileContent(path) {
  let content = readFileSync(path).toString();

  content = content.replaceAll(
    "https://dtapp-player.op.gg/adsense.txt",
    "https://gist.githubusercontent.com/MidKnightXI/7ecf3cdd0a5804466cb790855e2524ae/raw/9b88cf64f3bb955edfff27bdfba72f5181d8748b/remover.txt"
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
  writeFileSync(path, content);
}

async function rebuildAddDir(asarFilePath) {
  console.log("Unpacking OPGG asar file");
  extractAll(asarFilePath, "op-gg-unpacked");

  const assetDir = normalize("op-gg-unpacked/assets/react");
  const assetFiles = readdirSync(assetDir);

  for (let fileName of assetFiles) {
    if (fileName.endsWith(".js")) {
      console.log(`Patching: ${fileName}`);
      replaceAdFileContent(normalize(`${assetDir}/${fileName}`));
    }
  }

  console.log(`Rebuilding ${asarFilePath} without ads urls`);
  await createPackageWithOptions("op-gg-unpacked", asarFilePath, {
    unpackDir: "{node_modules/node-ovhook,node_modules/rust-process}",
  });

  console.log(`Deleted temporary directory`);
  sync("op-gg-unpacked");
}

function killOpgg() {
  console.log("Killing OPGG process");
  process.platform === "darwin"
    ? spawn("killall", ["-9", "OP.GG"])
    : spawn("taskkill", ["/im", "OP.GG.exe", "/F"]);
  return;
}

function main() {
  const asarFilePath =
    process.platform === "darwin"
      ? normalize("/Applications/OP.GG.app/Contents/Resources/app.asar")
      : normalize(
          `${dirname(
            process.env.APPDATA
          )}/Local/Programs/OP.GG/resources/app.asar`
        );

  if (!existsSync(asarFilePath)) {
    console.log(`Cannot find asar file at ${asarFilePath}`);
    return 84;
  }
  killOpgg();
  rebuildAddDir(asarFilePath);
}

main();
