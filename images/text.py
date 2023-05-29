from pyjoycon import JoyCon, get_L_id
import pyautogui, time
joycon_id = get_L_id()

joycon = JoyCon(*joycon_id)

while 1:
    
    shared_event = joycon.get_status()['buttons']['shared']
    btn_event = joycon.get_status()['buttons']['left']
    stick_event = joycon.get_status()['analog-sticks']['left']

    ## button shortcut keymap
    if btn_event['up']:  # eraser
        pyautogui.hotkey('e')
        continue
    if btn_event['right']: #pen
        pyautogui.hotkey('p')
        continue
    if btn_event['left']: #hude
        pyautogui.hotkey('b')
        continue
    if btn_event['down']:  #bokasi
        pyautogui.hotkey('j')
        continue

    if btn_event['l']:  #undo
        pyautogui.hotkey('command', 'z')
        continue
    if btn_event['zl']:  #redo
        pyautogui.hotkey('command','t')
        continue

    ## analog-sticks shortcut keymap
    if stick_event['vertical']>=3200: #max 3410 min 1315
        pyautogui.hotkey('command','=')
        continue
    if stick_event['vertical']<=1400: #max 3410 min 1315
        pyautogui.hotkey('command','-')
        continue
    if stick_event['horizontal']>=3000: #max 3250 min 954 
        pyautogui.hotkey('=')
        continue
    if stick_event['horizontal']<=1100: #max 3250 min 954 
        pyautogui.hotkey('-')
        continue
    

    #share button
    if shared_event['capture']: #max 3250 min 954 
        pyautogui.hotkey('m')
        continue


    time.sleep(0.001)
    