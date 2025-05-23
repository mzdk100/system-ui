#ifndef UIFW_PRIV_H
#define UIFW_PRIV_H

#include <jni.h>
#include "lib.h"
#include "../ui.h"
#include "../ui_android.h"

#ifndef LIBUI
#define LIBUI(ret, name) JNIEXPORT ret JNICALL Java_dev_danielc_libui_LibUI_##name
#endif

#define uiAreaSignature 0x41726561
#define uiBoxSignature 0x426F784C
#define uiButtonSignature 0x42746F6E
#define uiCheckboxSignature 0x43686B62
#define uiColorButtonSignature 0x436F6C42
#define uiComboboxSignature 0x436F6D62
#define uiDateTimePickerSignature 0x44545069
#define uiEditableComboboxSignature 0x45644362
#define uiEntrySignature 0x456E7472
#define uiFontButtonSignature 0x466F6E42
#define uiFormSignature 0x466F726D
#define uiGridSignature 0x47726964
#define uiGroupSignature 0x47727062
#define uiLabelSignature 0x4C61626C
#define uiMultilineEntrySignature 0x4D6C6E45
#define uiProgressBarSignature 0x50426172
#define uiRadioButtonsSignature 0x5264696F
#define uiSeparatorSignature 0x53657061
#define uiSliderSignature 0x536C6964
#define uiSpinboxSignature 0x5370696E
#define uiTabSignature 0x54616273
#define uiTableSignature 0x5461626C
#define uiWindowSignature 0x57696E64
#define uiScrollSignature 0x1234567

struct uiAndroidControl {
	uiControl c;
	jobject o;
	short request_width;
	short request_height;
	char is_activity;
};

typedef struct uiAndroidControl uiAndroidControl;

struct uiButton { struct uiAndroidControl c; };
struct uiLabel { struct uiAndroidControl c; };
struct uiWindow { struct uiAndroidControl c; };
struct uiBox { struct uiAndroidControl c; };
struct uiTab { struct uiAndroidControl c; };
struct uiProgressBar { struct uiAndroidControl c; };
struct uiSeparator { struct uiAndroidControl c; };
struct uiMultilineEntry { struct uiAndroidControl c; };
struct uiEntry { struct uiAndroidControl c; };
struct uiForm { struct uiAndroidControl c; };
struct uiScroll { struct uiAndroidControl c; };

#endif