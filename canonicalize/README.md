# `canonicalize`

`canonicalize` converts a HPGL file into a version where:

* `PU` and `PD` commands have no movement associated.
* each `PA` command only has one point associated.
* only absolute movements are used (unimplemented - currently, the script panics on relative movements)
