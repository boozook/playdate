SELECT DISTINCT fn.name, fn.low, fn.size, fn.hw_id as fn_hw_id, ln.low as ln_low, ln.hw_id as ln_hw_id, ln.lineno, fl.path
FROM functions fn
INNER JOIN lines ln
	ON fn.hw_id = ln.hw_id
	AND fn.low = ln.low -- OR (ln.low BETWEEN fn.low AND (fn.low + fn.size))
	AND (?1 BETWEEN fn.low AND (fn.low + fn.size))
INNER JOIN files fl
	ON ln.file_id = fl.id;

