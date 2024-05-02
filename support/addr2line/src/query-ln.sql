SELECT DISTINCT ln.low, ln.hw_id, ln.lineno, fl.path
FROM lines ln
INNER JOIN files fl
	ON ln.file_id = fl.id
	AND (?1 BETWEEN ln.low AND (ln.low + 4))


