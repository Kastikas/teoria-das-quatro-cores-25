import Lake
open Lake DSL

package "fourcolor25" where
  -- package config

lean_lib «FourColor» where
  -- add library configuration options here

@[default_target]
lean_exe «fourcolor25» where
  root := `Main
