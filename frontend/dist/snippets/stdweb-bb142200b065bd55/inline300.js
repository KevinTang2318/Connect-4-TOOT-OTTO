export function __cargo_web_snippet_270008deb209bd55fc7c158d5b41b31373341fef(Module, $0, $1, $2) { $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){var callback=($1);var state={cancelled:false,callback:callback};($2).then(function(value){if(! state.cancelled){callback(value,true);}},function(value){if(! state.cancelled){callback(value,false);}});return state;})()); }