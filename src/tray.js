/*
* @Author: DHJT
* @Date:   2025-01-11 09:48:54
* @Last Modified by:   DHJT
* @Last Modified time: 2025-01-11 09:53:26
*/
import {TrayIcon} from '@tauri-apps/api/tray';
import {Menu} from '@tauri-apps/api/menu';

export default async function tray_init() {
    const menu = await Menu.new({
        items: [
            {
                id: 'info',
                text: '关于',
                action: () => {
                    console.log("info press")
                }
            },
            {
                id: 'quit',
                text: '退出',
                action: () => {
                    // 退出逻辑
                    const appWindow = new Window('main');
                    appWindow.close()
                }
            },
        ],
    });

    const options = {
        icon: "D:\Workspaces\RustWork\tauri-app-demo\src-tauri\icons\icon.png",
        menu,
        menuOnLeftClick: false,
        // 托盘行为
        action: (event) => {
            switch (event.type) {
                case 'Click':
                    console.log(
                        `mouse ${event.button} button pressed, state: ${event.buttonState}`
                    );
                    break;
                case 'DoubleClick':
                    console.log(`mouse ${event.button} button pressed`);
                    break;
                case 'Enter':
                    console.log(
                        `mouse hovered tray at ${event.rect.position.x}, ${event.rect.position.y}`
                    );
                    break;
                case 'Move':
                    console.log(
                        `mouse moved on tray at ${event.rect.position.x}, ${event.rect.position.y}`
                    );
                    break;
                case 'Leave':
                    console.log(
                        `mouse left tray at ${event.rect.position.x}, ${event.rect.position.y}`
                    );
                    break;
            }
        },
    };

    const tray = await TrayIcon.new(options);
}
