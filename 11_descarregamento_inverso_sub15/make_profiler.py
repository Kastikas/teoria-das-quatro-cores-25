with open('11_descarregamento_inverso_sub15/discharge_universal.c', 'r') as f:
    code = f.read()

# 1. Update CheckIso prototypes
code = code.replace('void CheckIso(tp_confmat, tp_axle *, tp_vertices, int);', 'int CheckIso(tp_confmat, tp_axle *, tp_vertices, int);')
code = code.replace('void CheckIso();', 'int CheckIso();')
code = code.replace('void\nCheckIso(L, A, image, lineno)', 'int\nCheckIso(L, A, image, lineno)')

# 2. Add static variables
old_stat = """/* Universal question pool */
static int num_questions = 0;
static int noconf_base = 0;
static tp_question all_questions[MAXQUESTIONS];
static tp_confmat all_confmat[MAXQUESTIONS];
static int all_conf_id[MAXQUESTIONS];
static int all_conf_refl[MAXQUESTIONS];"""

new_stat = """/* Universal question pool */
static int num_questions = 0;
static int noconf_base = 0;
static tp_question all_questions[MAXQUESTIONS];
static tp_confmat all_confmat[MAXQUESTIONS];
static int all_conf_id[MAXQUESTIONS];
static int all_conf_refl[MAXQUESTIONS];

static unsigned int active_subset_mask = 0xFFFFFFFF;
static char conf_names[CONFS][MAXSTR];
static char last_conf_name[MAXSTR];
static int run_mode = 0; /* 0=normal, 1=profile, 2=test */
static char out_json_path[MAXSTR] = "";

typedef struct {
    int lineno;
    int lev;
    int hub_deg;
    unsigned int mask;
} tp_rline_prof;

static tp_rline_prof prof_list[5000];
static int num_prof = 0;
static int failed_rline_count = 0;"""

assert old_stat in code, "old_stat not found"
code = code.replace(old_stat, new_stat)

# 3. Add argument parsing in main
old_main_args = """   printmode = prtline = 0;
   if (ac < 4) { // jps"""

new_main_args = """   printmode = prtline = 0;
   if (ac >= 2 && strcmp(av[1], "--profile") == 0) {
      run_mode = 1;
      if (ac < 5) {
         fprintf(stderr, "Usage: %s --profile <presentation> <conf> <rules> [out_json]\\n", av[0]);
         exit(1);
      }
      strcpy(fname, av[2]);
      strcpy(UNAVSET, av[3]);
      strcpy(RULEFILE, av[4]);
      if (ac >= 6) strcpy(out_json_path, av[5]);
      else sprintf(out_json_path, "profile_%s.json", fname);
   } else if (ac >= 2 && strcmp(av[1], "--test") == 0) {
      run_mode = 2;
      if (ac < 6) {
         fprintf(stderr, "Usage: %s --test <mask_hex> <presentation> <conf> <rules>\\n", av[0]);
         exit(1);
      }
      active_subset_mask = (unsigned int) strtoul(av[2], NULL, 0);
      strcpy(fname, av[3]);
      strcpy(UNAVSET, av[4]);
      strcpy(RULEFILE, av[5]);
   } else if (ac < 4) { // jps"""

assert old_main_args in code, "old_main_args not found"
code = code.replace(old_main_args, new_main_args)

# 4. Modify case 'R'
old_case_r = """      case 'R':
	 if (Reduce(A, lineno, print >= PRTBAS ? 1 : 0) != 1)
	    Error("Reducibility failed", lineno);
	 break;"""

new_case_r = """      case 'R':
	 if (run_mode == 1) { /* profiling mode */
	    unsigned int mask = 0;
	    int c;
	    for (c = 0; c < noconf_base; c++) {
	       active_subset_mask = (1U << c);
	       if (Reduce(A, lineno, 0) == 1)
		  mask |= (1U << c);
	    }
	    active_subset_mask = 0xFFFFFFFF;
	    if (num_prof < 5000) {
	       prof_list[num_prof].lineno = lineno;
	       prof_list[num_prof].lev = lev;
	       prof_list[num_prof].hub_deg = deg;
	       prof_list[num_prof].mask = mask;
	       num_prof++;
	    }
	 } else if (run_mode == 2) { /* test mode */
	    if (Reduce(A, lineno, print >= PRTBAS ? 1 : 0) != 1) {
	       failed_rline_count++;
	       if (failed_rline_count <= 5)
		  (void) printf("  [FAIL] Line %d (level %d): Not reducible under mask 0x%x\\n",
				lineno, lev, active_subset_mask);
	    }
	 } else {
	    if (Reduce(A, lineno, print >= PRTBAS ? 1 : 0) != 1)
	       Error("Reducibility failed", lineno);
	 }
	 break;"""

assert old_case_r in code, "old_case_r not found"
code = code.replace(old_case_r, new_case_r)

# 5. Modify Q.E.D. and output
old_qed = """   /* final check */
   lineno = Getstring(str);
   ch = str;
   if (*ch++ != 'Q' || *ch++ != '.' || *ch != 'E')
      Error("`Q.E.D.' expected", lineno);
   (void) printf("%s verified successfully with 0 deficit! [24 CONFIGURATIONS CERTIFIED]\\n", fname);
   fflush(stdout);
   return (0);"""

new_qed = """   /* final check */
   if (run_mode == 1) {
      FILE *fout = fopen(out_json_path, "w");
      if (fout != NULL) {
         int i, c;
         fprintf(fout, "{\\n  \\\"presentation\\\": \\\"%s\\\",\\n  \\\"hub_degree\\\": %d,\\n  \\\"total_rlines\\\": %d,\\n  \\\"total_confs\\\": %d,\\n  \\\"confs\\\": [\\n",
                 fname, deg, num_prof, noconf_base);
         for (c = 0; c < noconf_base; c++) {
            fprintf(fout, "    \\\"%s\\\"%s\\n", conf_names[c], (c == noconf_base - 1) ? "" : ",");
         }
         fprintf(fout, "  ],\\n  \\\"rlines\\\": [\\n");
         for (i = 0; i < num_prof; i++) {
            fprintf(fout, "    {\\\"line\\\": %d, \\\"level\\\": %d, \\\"mask\\\": %u}%s\\n",
                    prof_list[i].lineno, prof_list[i].lev, prof_list[i].mask, (i == num_prof - 1) ? "" : ",");
         }
         fprintf(fout, "  ]\\n}\\n");
         fclose(fout);
         printf("[PROFILER] Profile written to %s (%d R-lines profiled).\\n", out_json_path, num_prof);
      }
      return (0);
   } else if (run_mode == 2) {
      if (failed_rline_count == 0) {
         printf("[TEST] %s: 100%% SUCCESS (0 open axles) with mask 0x%x\\n", fname, active_subset_mask);
         return (0);
      } else {
         printf("[TEST] %s: FAILED (%d open axles out of %d) with mask 0x%x\\n", fname, failed_rline_count, num_prof, active_subset_mask);
         return (1);
      }
   }
   lineno = Getstring(str);
   ch = str;
   if (*ch++ != 'Q' || *ch++ != '.' || *ch != 'E')
      Error("`Q.E.D.' expected", lineno);
   (void) printf("%s verified successfully with 0 deficit! [24 CONFIGURATIONS CERTIFIED]\\n", fname);
   fflush(stdout);
   return (0);"""

assert old_qed in code, "old_qed not found"
code = code.replace(old_qed, new_qed)

# 6. Modify Reduce question filtering and CheckIso call
old_reduce_loop = """      for (h = 0; h < num_questions; ++h)
	 if (SubConf(adjmat, B->upp, all_questions[h], edgelist, image))
	    break;
      if (h == num_questions) {
	 if (print)
	    (void) printf("Not reducible\\n");
	 return (0);
      }
      /* Semi-reducibility test found h-th question appearing */
      redverts = all_questions[h][1].u;
      redring = all_questions[h][1].v;

      if (print) {
         int cid = all_conf_id[h];
	 (void) printf("Conf(%d,%d,%d) [Q#%d, refl=%d]: ",
                cid / 70 + 1, (cid % 70) / 7 + 1, cid % 7 + 1, h, all_conf_refl[h]);
	 for (j = 1; j <= redverts; j++) {
	    if (image[j] != -1)
	       (void) printf(" %d(%d)", image[j], j);
	 }
	 (void) printf("\\n");
      }
      /* Double-check isomorphism against the exact configuration matrix */
      CheckIso(all_confmat[h], B, image, lineno);"""

new_reduce_loop = """      for (h = 0; h < num_questions; ++h) {
	 if ((active_subset_mask & (1U << all_conf_id[h])) == 0)
	    continue;
	 if (SubConf(adjmat, B->upp, all_questions[h], edgelist, image)) {
	    if (CheckIso(all_confmat[h], B, image, lineno))
	       break;
	 }
      }
      if (h == num_questions) {
	 if (print)
	    (void) printf("Not reducible\\n");
	 return (0);
      }
      /* Semi-reducibility test found h-th question appearing */
      redverts = all_questions[h][1].u;
      redring = all_questions[h][1].v;

      if (print) {
         int cid = all_conf_id[h];
	 (void) printf("Conf(%d,%d,%d) [Q#%d, refl=%d]: ",
                cid / 70 + 1, (cid % 70) / 7 + 1, cid % 7 + 1, h, all_conf_refl[h]);
	 for (j = 1; j <= redverts; j++) {
	    if (image[j] != -1)
	       (void) printf(" %d(%d)", image[j], j);
	 }
	 (void) printf("\\n");
      }"""

assert old_reduce_loop in code, "old_reduce_loop not found"
code = code.replace(old_reduce_loop, new_reduce_loop)

# 7. Replace errors inside CheckIso with return 0
code = code.replace('Error("Isomorphism error 1", lineno);', 'return 0;')
code = code.replace('Error("Isomorphism error 2", lineno);', 'return 0;')
code = code.replace('Error("Isomorphism error 3", lineno);', 'return 0;')
code = code.replace('Error("Isomorphism error 4", lineno);', 'return 0;')
code = code.replace('Error("Isomorphism error 5", lineno);', 'return 0;')
code = code.replace('Error("Isomorphism error 6", lineno);', 'return 0;')
code = code.replace('Error("Unexpected error in CheckIso", lineno);', 'return 0;')

old_indu = """#define INDUCHECK(aa, bb, cc) if(aa && bb && cc!=1)      \\
        Error("Isomorphism not induced",lineno);"""
new_indu = """#define INDUCHECK(aa, bb, cc) if(aa && bb && cc!=1) return 0;"""
assert old_indu in code, "old_indu not found"
code = code.replace(old_indu, new_indu)

old_checkiso_end = """      INDUCHECK(used[e], used[b], X[e][b]);
   }
}/* CheckIso */"""

new_checkiso_end = """      INDUCHECK(used[e], used[b], X[e][b]);
   }
   return 1;
}/* CheckIso */"""

assert old_checkiso_end in code, "old_checkiso_end not found"
code = code.replace(old_checkiso_end, new_checkiso_end)

# 8. Store conf names in ReadConf and UniversalGetConf
old_readconf = """   name[0] = '\\0';
   t = name;
   while (*t == '\\0' || *t == '\\n') {
      if (fgets(name, sizeof(name), F) == NULL)
	 return ((long) 1);
      for (t = name; *t == ' ' || *t == '\\t'; t++);
   }"""

new_readconf = """   name[0] = '\\0';
   t = name;
   while (*t == '\\0' || *t == '\\n') {
      if (fgets(name, sizeof(name), F) == NULL)
	 return ((long) 1);
      for (t = name; *t == ' ' || *t == '\\t'; t++);
   }
   {
      char *p;
      for (p = name; *p; p++) if (*p == '\\r' || *p == '\\n') *p = '\\0';
      strncpy(last_conf_name, name, sizeof(last_conf_name) - 1);
   }"""

assert old_readconf in code, "old_readconf not found"
code = code.replace(old_readconf, new_readconf)

old_unigetconf = """      Radius(current_conf);
      noconf_base++;"""

new_unigetconf = """      Radius(current_conf);
      strncpy(conf_names[noconf_base], last_conf_name, sizeof(conf_names[0]) - 1);
      noconf_base++;"""

assert old_unigetconf in code, "old_unigetconf not found"
code = code.replace(old_unigetconf, new_unigetconf)

with open('11_descarregamento_inverso_sub15/discharge_profiler.c', 'w') as f:
    f.write(code)

print("discharge_profiler.c successfully generated!")
