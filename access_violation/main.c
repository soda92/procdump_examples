#include <windows.h>

// Function to handle button clicks
void OnButtonClick(HWND hwnd) {
  MessageBox(hwnd, "Hello, Win32!", "Greeting", MB_OK | MB_ICONINFORMATION);
}

// Window procedure
LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam,
                            LPARAM lParam) {
  static HWND
      button; // Declare button as static to preserve its value between messages

  switch (uMsg) {
  case WM_CREATE: {
    // Create button
    button =
        CreateWindow("BUTTON",                              // Predefined class
                     "Click Me",                            // Button text
                     WS_VISIBLE | WS_CHILD | BS_PUSHBUTTON, // Styles
                     100, 100, 100, 30,                     // Position and size
                     hwnd,                                  // Parent window
                     (HMENU)1,                              // ID of the button
                     ((LPCREATESTRUCT)lParam)->hInstance,   // Instance handle
                     NULL); // Additional application data
    if (button == NULL) {
      return -1;
    }
    return 0;
  }
  case WM_COMMAND:
    if (LOWORD(wParam) == 1 && HIWORD(wParam) == BN_CLICKED) {
      // Button click event
      OnButtonClick(hwnd);
    }
    break;
  case WM_DESTROY:
    PostQuitMessage(0); // Exit the application
    break;
  default:
    return DefWindowProc(hwnd, uMsg, wParam, lParam);
  }
  return 0;
}

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance,
                   LPSTR lpCmdLine, int nCmdShow) {
  // Register the window class
  const char CLASS_NAME[] = "SampleWindowClass";
  WNDCLASS wc = {};
  wc.lpfnWndProc = WindowProc;
  wc.hInstance = hInstance;
  wc.lpszClassName = CLASS_NAME;
  RegisterClass(&wc);

  // Create the window
  HWND hwnd = CreateWindowEx(0,                   // Optional window styles
                             CLASS_NAME,          // Window class
                             "Win32 GUI Demo",    // Window title
                             WS_OVERLAPPEDWINDOW, // Window style
                             CW_USEDEFAULT, CW_USEDEFAULT, 320,
                             240,       // Position and size
                             NULL,      // Parent window
                             NULL,      // Menu
                             hInstance, // Instance handle
                             NULL);     // Additional application data

  if (hwnd == NULL) {
    return 0;
  }

  ShowWindow(hwnd, nCmdShow);

  // Message loop
  MSG msg = {};
  while (GetMessage(&msg, NULL, 0, 0)) {
    TranslateMessage(&msg);
    DispatchMessage(&msg);
  }

  return 0;
}