# sentinel

*sentinel* is a simple, extensible, and [gizmodic](https://gizmodic.software) directory watcher daemon.

it is configured in **lua**, allowing you to specify what directories you want to watch, and what you want to happen whenever a certain event takes place.

## configuration

to configure *sentinel*, create `~/.config/sentinel/init.lua` and write your configuration.<br/>
`recurse` sets whether or not you want the directory to be recursively watched.<br/>
each of the others are functions that get run whenever their respective event takes place in the watched directory.
> the `filepaths` variable is a table of all file paths that were affected by the event.

```lua
return {
  ["/foo/bar/baz"] = {                                    -- the canonical (absolute) path of the directory to watch
    ["recurse"] = true,                                   -- whether or not to recursively watch the directory
    ["access"] = function(filepaths) print("access") end, -- function to run when an 'access' event occurs
    ["create"] = function(filepaths) print("create") end, -- function to run when a 'create' event occurs
    ["modify"] = function(filepaths) print("modify") end, -- function to run when a 'modify' event occurs
    ["remove"] = function(filepaths) print("remove") end, -- function to run when a 'remove' event occurs
    ["other"]  = function(filepaths) print("other") end,  -- function to run when an 'other' event occurs. this refers to any operation that doesn't match the above
  },
}
```
