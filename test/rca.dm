#ifdef LINUX
	#define RAYON_CA "./lib.so"
#else
	#define RAYON_CA "./lib.dll"
#endif
#define RCA(fun) call_ext(RAYON_CA, fun)
#define RCA_VERBOSE(fun) world.log << "[fun]: " + RCA(fun)
#ifdef VERBOSE
	#define debug_proc world.log << "[splittext("[__PROC__]", "/")[4]]: [json_encode(args)] - [.]"
#else
	#define debug_proc
#endif

/// Generates X by Y field with CA in {steps} steps.
/proc/CA_steps(x, y, steps, alive_prob = 0.5)
	return RCA("create_and_process")("[x]", "[y]", "[steps]", "[alive_prob * 100]")

/rca
	var/rca_path = RAYON_CA
	var/list/automatons = list()
	proc/create_automaton(id)
		. = RCA("create_automaton")(id)
		debug_proc
		if(. == "OK")
			automatons |= id
		else
			CRASH("ERR: create_automaton([id]) => [.]")
		return .

	proc/set_strat(id, strat)
		. = RCA("set_strat")(id, strat)
		debug_proc

	proc/set_xy(id, x, y)
		. = RCA("set_xy")(id, "[x]", "[y]")
		debug_proc

	proc/randomize(id, alive_prob as num)
		. = RCA("randomize")(id, "[alive_prob * 100]")
		debug_proc

	proc/make_steps(id as text, steps as num)
		. = RCA("make_steps")(id, num2text(steps))
		debug_proc

	proc/get_field(id)
		var/out = RCA("get_field_raw")(id)
		var/list/rows = splittext(out, ";")
		var/list/field = list()
		for(var/r in rows)
			var/row = splittext(r, " ")
			var/row_res = list()
			for(var/i in row)
				row_res += text2num(i)
			field += list(row_res)
		debug_proc
		return field
