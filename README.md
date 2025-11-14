# Sympatica: API and Frontend for the Empatica E4 Wristband
For questions or comments you can contact me [here](mailto:e.t.unal@se23.qmul.ac.uk).


#### What is this?
Since the original application and cloud solution for this product was sunset in early 2025, I figured I'd port it to a open-source application. This (work-in-progress, barely complete) project is a web-based UI for the E4 built using Vue and Django. 

#### Why?
Made partly to use in my undergraduate final-year project, partly for my ECS639U Web Programming module, and mostly to spite Empatica and open source this bit of gear.

I humbly ask for your patience while I get this to a runnable state. 

## Features
* Real-time monitoring of Blood Volume Pulse, Electrodermal Activity, Skin Temperature and 3-Axis Accelerometer sensors.
* Save Patients, Sessions for each Patient, and Readings for each Session
    * Export feature coming soon (will be in Empatica format)
* View past sessions with interactive graphs.

## Requirements
- npm, django, python 3.14
- Bluetooth-enabled computer (tested on Linux 6.17 kernel and macOS 15.6)
- Chrome, as this program relies on the Web Bluetooth API 
    - note: if you're using linux you'll have to go into `chrome://flags` and manually enable Web Bluetooth as it's currently an experimental feature.

## Run Locally
Clone the project

```bash
  git clone https://github.com/mountainaught/sympatica
```

Go to the project directory

```bash
  cd sympatica
```
#### Frontend:


Install dependencies and start the server

```bash
  npm install
  npm run start
```

#### Backend:
Create a virtual environment, activate it, and install the requirements 
```bash
python -m venv sympatica
source myworld/bin/activate
pip install -r sympatica/requirements.txt

```
Then start the Django server:
```bash
python sympatica/manage.py runserver 8000
```
### Known issues
* Bluetooth connection management is a bit messy, disconnect between sessions or readings get jumbled up.
* Some screen elements get cut off on smaller resolutions.
* Alignment between elements is crooked.
