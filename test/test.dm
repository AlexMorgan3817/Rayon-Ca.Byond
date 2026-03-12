/proc/print_field(rca/rca, automaton)
	var/res = rca.get_field(automaton)
	if(res[1] == "@")
		CRASH("Failed to get_field: [res]")
	. = ""
	for(var/row in res)
		for(var/i in row)
			// . += "[i]"
			if(i)
				. += "$"
			else
				. += " "
		. += "\n"
	world.log << .

/proc/test_oneshot(rca/rca)
	var/start_time = world.realtime
	world.log << "Starting one-shot test: [start_time]"
	// spawn(0)
	var/res = CA_steps(5, 20, 20, 0.5, "conway")
	// world.log << "Elapsed time: [world.realtime] Delta - [world.realtime - start_time]"
	if(res[1] == "@")
		CRASH("Oneshot test failed: [res]")
	for(var/i in json_decode(res))
		world.log << json_encode(i)

/proc/test_manual(rca/rca)
	rca.create_automaton("test2")
	rca.create_automaton("test2")
	rca.create_automaton("test")
	rca.set_xy("test", 5, 25)
	rca.randomize("test", 0.5)
	print_field(rca, "test")
	rca.make_steps("test", 10)
	print_field(rca, "test")

/proc/test_two_strats(rca/rca, id = "2strats")
	rca.create_automaton(id)

	rca.set_xy(id, 25, 50)
	rca.randomize(id, 0.5)
	print_field(rca, id)

	if(rca.set_strat(id, "amogus") == "OK")
		rca.make_steps(id, 100)
		print_field(rca, id)

	if(rca.set_strat(id, "corridors") == "OK")
		rca.make_steps(id, 100)
		print_field(rca, id)

	if(rca.set_strat(id, "conway") == "OK")
		rca.make_steps(id, 1)
		print_field(rca, id)

/world/New()
	var/rca/rca = new/rca()
	world.log << "Rayon-CA.BYOND version: " + call_ext(RAYON_CA, "get_version")()
	world.log << "\t # oneshot"
	test_oneshot(rca)
	world.log << "\t # manual"
	test_manual(rca)
	world.log << "\t # two strats"
	test_two_strats(rca, "2strats")
	//shutdown(src.address)
	//Del()
