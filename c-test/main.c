#include "../target/debug/circleci.h"
#include <stdio.h>

int main() {
	// Ensure you've got your CircleCI token loaded in the environment.
	char *apikey = getenv("CIRCLE_TOKEN");
	if (apikey == NULL) {
		printf("No API key available, did you set $CIRCLE_TOKEN?\n");
		return 1;
	}
	Api *api = circleci_api("https://circleci.com/api", apikey);
	CMe me = circleci_api_me(api);

	// CCollaboration collabs[255];
	// circleci_api_collaborations(api, collabs, 255);
	printf(me.id);

	return 0;
}
