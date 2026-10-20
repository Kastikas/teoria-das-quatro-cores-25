#define main discharge_main
#include "../src/discharge.c"
#undef main

int main(int argc, char **argv) {
    if (argc < 3) {
        fprintf(stderr, "Usage: %s <axle.json> <extracted.conf>\n", argv[0]);
        return 1;
    }

    FILE *f_axle = fopen(argv[1], "r");
    if (!f_axle) {
        fprintf(stderr, "Cannot open axle file %s\n", argv[1]);
        return 1;
    }

    char line[4096];
    if (!fgets(line, sizeof(line), f_axle)) {
        fprintf(stderr, "Cannot read axle line\n");
        fclose(f_axle);
        return 1;
    }
    fclose(f_axle);

    // Parse low and upp arrays from JSON line
    tp_axle B;
    memset(&B, 0, sizeof(B));

    char *p_low = strstr(line, "\"low\":[");
    char *p_upp = strstr(line, "\"upp\":[");
    if (!p_low || !p_upp) {
        fprintf(stderr, "Invalid axle JSON format\n");
        return 1;
    }

    p_low += 7;
    int idx = 0;
    while (*p_low && *p_low != ']' && idx < CARTVERT) {
        B.low[idx++] = atoi(p_low);
        while (*p_low && *p_low != ',' && *p_low != ']') p_low++;
        if (*p_low == ',') p_low++;
    }

    p_upp += 7;
    idx = 0;
    while (*p_upp && *p_upp != ']' && idx < CARTVERT) {
        B.upp[idx++] = atoi(p_upp);
        while (*p_upp && *p_upp != ',' && *p_upp != ']') p_upp++;
        if (*p_upp == ',') p_upp++;
    }

    int deg = B.low[0];
    printf("[VERIFY_EXTRACTED] Axle degree: %d\n", deg);

    // Allocate memory for conf and redquestions
    conf = (tp_confmat *) malloc(CONFS * sizeof(tp_confmat));
    redquestions = (tp_question *) malloc(CONFS * sizeof(tp_question));
    if (!conf || !redquestions) {
        fprintf(stderr, "Out of memory\n");
        return 1;
    }

    // Set UNAVSET to argv[2]
    strncpy(UNAVSET, argv[2], sizeof(UNAVSET) - 1);
    UNAVSET[sizeof(UNAVSET) - 1] = '\0';

    noconf = GetConf(conf, redquestions);
    printf("[VERIFY_EXTRACTED] Loaded %d extracted configurations from %s\n", noconf, UNAVSET);
    if (noconf <= 0) {
        fprintf(stderr, "No configurations loaded\n");
        return 1;
    }

    tp_adjmat adjmat;
    tp_edgelist edgelist;
    tp_vertices image;

    Getadjmat(&B, adjmat);
    GetEdgelist(&B, edgelist);

    int failed = 0;
    for (int h = 0; h < noconf; h++) {
        memset(image, 0, sizeof(image));
        int sub_ok = SubConf(adjmat, B.upp, redquestions[h], edgelist, image);
        if (!sub_ok) {
            fprintf(stderr, "[FAIL] Conf %d failed SubConf!\n", h + 1);
            failed++;
            continue;
        }

        // Run CheckIso to verify orientation, facial consistency, and induced subgraph
        CheckIso(conf[h], &B, image, 1);
        printf("[OK] Conf %d (verts=%ld, ring=%ld): SubConf=1, CheckIso=OK\n",
               h + 1, conf[h][0][0], conf[h][0][1]);
    }

    if (failed > 0) {
        fprintf(stderr, "[VERIFY_EXTRACTED] FAILED: %d / %d configurations failed verification!\n", failed, noconf);
        return 2;
    }

    printf("[VERIFY_EXTRACTED] SUCCESS: 100%% of extracted configurations (%d/%d) verified with SubConf=1 and CheckIso=OK!\n",
           noconf, noconf);
    return 0;
}
