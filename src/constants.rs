pub const REACT_EXCLUDE: [(&str, &str); 12] = [
    ("boolean", "src"),
    ("collections", "src"),
    ("console", "src"),
    ("instance-of", "src"),
    ("luau-polyfill", "src"),
    ("math", "src"),
    ("number", "src"),
    ("promise", "lib"),
    ("react-is", ""),
    ("shared", ""),
    ("string", "src"),
    ("timers", "src"),
];

pub const REACT_INCLUDE: [(&str, &str); 3] = [
    ("react", ""),
    ("react-reconciler", ""),
    ("react-test-renderer", ""),
];

pub const JEST_INCLUDE: [&str; 6] = [
    "DiffSequences",
    "Emittery",
    "Expect",
    "Path",
    "PrettyFormat",
    "Throat",
];

pub const REACT_PATH: &str =
    "script.Parent.Parent.Parent.Parent.ReactLua.node_modules[\"@jsdotlua\"]";

pub const RESULT_DIR: &str = "./JestLua/node_modules/@jsdotlua";
