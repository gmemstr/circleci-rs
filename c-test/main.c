#include "../target/debug/circleci.h"
#include <stdio.h>

int main() {
	// Ensure you've got your CircleCI token loaded in the environment.
	char *apikey = getenv("CIRCLE_TOKEN");
	if (apikey == NULL) {
		printf("No API key available, did you set $CIRCLE_TOKEN?\n");
		return 1;
	}
	Api *api = api_v1("https://circleci.com/api/v1.1", apikey);
	CMe me = api_v1_me(api);
	printf(me.login, "\n");
	return 0;
}

