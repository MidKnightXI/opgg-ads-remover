// module.exports.extractAll = function (archive, dest) {
//     const filesystem = disk.readFilesystemSync(archive)
//     const filenames = filesystem.listFiles()
//     // under windows just extract links as regular files
//     const followLinks = process.platform === 'win32'
//     // create destination directory
//     fs.mkdirpSync(dest)
//     for (const fullPath of filenames) {
//       // Remove leading slash
//       const filename = fullPath.substr(1)
//       const destFilename = path.join(dest, filename)
//       const file = filesystem.getFile(filename, followLinks)
//       if (file.files) {
//         // it's a directory, create it and continue with the next entry
//         fs.mkdirpSync(destFilename)
//       } else if (file.link) {
//         // it's a symlink, create a symlink
//         const linkSrcPath = path.dirname(path.join(dest, file.link))
//         const linkDestPath = path.dirname(destFilename)
//         const relativePath = path.relative(linkDestPath, linkSrcPath)
//         // try to delete output file, because we can't overwrite a link
//         try {
//           fs.unlinkSync(destFilename)
//         } catch {}
//         const linkTo = path.join(relativePath, path.basename(file.link))
//         fs.symlinkSync(linkTo, destFilename)
//       } else {
//         // it's a file, extract it
//         const content = disk.readFileSync(filesystem, filename, file)
//         fs.writeFileSync(destFilename, content)
//         if (file.executable) {
//           fs.chmodSync(destFilename, '755')
//         }
//       }
//     }
//   }