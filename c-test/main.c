#include "../target/debug/circleci.h"
#include <stdio.h>
#include <stdint.h>

int main() {
	// Ensure you've got your CircleCI token loaded in the environment.
	char *apikey = getenv("CIRCLE_TOKEN");
	if (apikey == NULL) {
		printf("No API key available, did you set $CIRCLE_TOKEN?\n");
		return 1;
	}
	Api *api = circleci_api("https://circleci.com/api", apikey);
	CMe *me = circleci_api_me(api);

	printf("Username: %s\n", me->login);

	int len;
	CCollaboration *collabs = circleci_api_collaborations(api, &len);
	printf("Total orgs: %d\n", len);
	for (int i=0; i<len; i++) {
		printf("Collaboration %d: %s\n", i+1, collabs[i].name);
	}

	int len2;
	CProject *projects = circleci_api_projects(api, &len2);
	printf("Total projects: %d\n", len2);
	for (int i=0; i<len2; i++) {
		printf("Project %d: %s\n", i+1, projects[i].reponame);
	}

	return 0;
}
