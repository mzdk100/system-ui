#include <stdlib.h>
#include "uipriv_android.h"

static uintptr_t control_handle(uiControl *c) {
	return (uintptr_t)((uiAndroidControl *)c)->o;
}

static void control_show(uiControl *c) {
	if (c->Signature == uiWindowSignature) {
		popupwindow_open(get_jni_env(), get_jni_ctx(), uiViewFromControl(c));
	}
}

static void control_hide(uiControl *c) {
	// TODO
}

static void control_disable(uiControl *c) {
	view_set_view_enabled(uiViewFromControl(c), 0);
}

static void control_enable(uiControl *c) {
	view_set_view_enabled(uiViewFromControl(c), 1);
}

static void control_destroy(uiControl *c) {
	view_destroy(uiViewFromControl((c)));
	free(c); // We only will free the parent wrapper :(
	// TODO: Track children/parents for views
}

struct uiAndroidControl *new_view_control(int signature) {
	struct uiAndroidControl *b = calloc(1, sizeof(struct uiAndroidControl));
	b->c.Signature = signature;
	b->c.Handle = (uintptr_t (*)(uiControl *)) uiViewFromControl;
	b->c.Disable = control_disable;
	b->c.Enable = control_enable;
	b->c.Show = control_show;
	b->c.Hide = control_hide;

	b->c.Destroy = control_destroy;

	// TODO:
	//	uiControl *(*Parent)(uiControl *);
	//	void (*SetParent)(uiControl *, uiControl *);
	//	int (*Toplevel)(uiControl *);
	//	int (*Visible)(uiControl *);
	//	int (*Enabled)(uiControl *);

	return b;
}

uiControl *uiControlFromView(jobject obj) {
	JNIEnv *env = get_jni_env();
	int signature = uiLabelSignature;
	if ((*env)->IsInstanceOf(env, obj, (*env)->FindClass(env, "android/widget/Button"))) {
		signature = uiButtonSignature;
	} else if ((*env)->IsInstanceOf(env, obj, (*env)->FindClass(env, "android/widget/TextView"))) {
		signature = uiLabelSignature;
	}

	struct uiAndroidControl *ctl = new_view_control(uiButtonSignature);
	ctl->o = obj;

	return uiControl(ctl);
}

uiControl *uiControlFromID(const char *id) {
	JNIEnv *env = get_jni_env();
	jobject obj = (*env)->NewGlobalRef(env, view_get_by_id(env, get_jni_ctx(), id));
	return (uiControl *)uiControlFromView(obj);
}
