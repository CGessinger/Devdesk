
<div align="center">
   <img alt="App Icon" src="/src-tauri/icons/icon.png" height="150">
   
   [![Windows Support](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/publicsoftwaredeploy/Public-Project-Assistant/) 
   [![Ubuntu Support](https://img.shields.io/badge/Ubuntu-E95420?style=for-the-badge&logo=ubuntu&logoColor=white)](https://github.com/publicsoftwaredeploy/Public-Project-Assistant/) 
   [![Arch Linux Support](https://img.shields.io/badge/Arch_Linux-1793D1?style=for-the-badge&logo=arch-linux&logoColor=white)](https://github.com/publicsoftwaredeploy/Public-Project-Assistant/) 
   [![MacOS Support](https://img.shields.io/badge/MACOS-adb8c5?style=for-the-badge&logo=macos&logoColor=white)](https://github.com/publicsoftwaredeploy/Public-Project-Assistant/)
   
</div>


# Public Project Assistant

An easy-to-use gui overlay to represent your project file structure.

### Concept

The idea is to have multiple **Main Directories** that contain many **Types**. Each type is represented by a subdirectory in its parent folder. 

For Example: you have a main programming directory in your document folder, called *Programming*. In that folder you can have multiple types of projects like *Release*, *Concept*, *Sanbox* etc.

Your file structure could look like:

1. Programming

   1. Release

      1. ProjectA

      2. ProjectB

   2. Concept

      1. ProjectC

   3. Sanbox

      1. ProjectD
      
### Features

* [x] List all of your projects and search through them

* [x] Create new projects

* [x] Open your terminal and editor on one of your projects

* [x] Light and Darkmode support!

* [x] Configuration

#### Experimental Features

You can enable these features in the app by going into `settings > Enable Experimental Features`. These features are in an early state of implementation and not fully tested yet.

* [x] Display stats of technologies used in your project

* [x] Integrate Make button if makefile exists

* [ ] Git clone integration when creating new project (Temporarily disabled)

#### ToDo

* [ ] Integrate Npm run option when package file exists

### Download and Installation

Public Project Assistant is currently supported for Windows and Mac. Linux is not fully tested, so you might run into issues there.

Executables will be available on the first release. Please feel free to use the beta version already by following these steps:

`git clone https://github.com/publicsoftwaredeploy/Public-Project-Assistant`

`npm install`

`npm run tauri build`

Make sure you have rust installed and all [tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) properly setup!

### Contribute

Every contribution and feedback is very welcome! Just fork the repo. Clone it to your local machine and run:

`npm install`

`npm run tauri dev`

Make sure you have rust installed and all [tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) properly setup!
