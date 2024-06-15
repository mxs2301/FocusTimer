from pathlib import Path
import os
import platform
import shutil

def setup():

    if (platform.system() == "Linux"):
        user = os.getenv("USER")
        path = "/home/" + user + "/.config/FocusTimer/"
        result = os.path.exists(path)

        if(result == True):
            linux_setup(path)
            print("All done")
        else:
            os.mkdir(path)
            setup()
    else:
        print("Not Linux")
        print("Still a todo, to figure out windows")
      

def linux_setup(path):
    os.mkdir(path +"/audio")
    shutil.copy(os.getcwd() + "/media/success.mp3", path + "/audio")

# def windows_setup(path):
    # os.mkdir(path + "\\audio")
    
setup()
