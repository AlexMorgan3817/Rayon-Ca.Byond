#define RAYON_CA "./lib.so"
/// Generates X by Y field with CA in {steps} steps.
/proc/CA_steps(x, y, steps, alive_prob = 0.5)
	if(0 < alive_prob && alive_prob < 1)
		alive_prob *= 100
	return call_ext(RAYON_CA, "create_and_process")("[x]", "[y]", "[steps]", "[alive_prob]")

/world/New()
	var/res = null
	try
		res = CA_steps(40, 25, 100)
	catch(var/exception/e)
		world.log << "[e]"
	for(var/i in json_decode(res))
		world.log << json_encode(i)
	//shutdown(src.address)
	Del()
