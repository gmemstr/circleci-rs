#include <gtk/gtk.h>
#include "../target/debug/circleci.h"
#include <stdio.h>

CMe circleci_username(char *apikey) {
    // Ensure you've got your CircleCI token loaded in the environment.

    Api *api = api_v1("https://circleci.com/api", apikey);
    CMe me = api_v1_me(api);

    return me;
}

static void activate (GtkApplication *app, gpointer user_data) {
    GtkWidget *window;
    GtkWidget *button;

    window = gtk_application_window_new (app);
    gtk_window_set_title (GTK_WINDOW (window), "Window");
    gtk_window_set_default_size (GTK_WINDOW (window), 1280, 720);


    char *apikey = getenv("CIRCLE_TOKEN");
    if (apikey == NULL) {
        printf("No API key available, did you set $CIRCLE_TOKEN?\n");
        return;
    }
    char * prefix = "CircleCI Username: ";
    CMe me = circleci_username(apikey);
    char buf[256];
    snprintf(buf, sizeof(buf), "%s%s", prefix, me.login);

    GtkWidget* username = gtk_label_new (buf);
    gtk_window_set_child (GTK_WINDOW (window), username);
    gtk_window_present (GTK_WINDOW (window));
}

int main (int argc, char **argv) {
  GtkApplication *app;
  int status;

  app = gtk_application_new ("com.gabrielsimmer.circleci", G_APPLICATION_FLAGS_NONE);
  g_signal_connect (app, "activate", G_CALLBACK (activate), NULL);
  status = g_application_run (G_APPLICATION (app), argc, argv);
  g_object_unref (app);

  return status;
}