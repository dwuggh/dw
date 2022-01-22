
use builtin;
use str;

set edit:completion:arg-completer[dw] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'dw'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'dw'= {
            cand --generate-shell-completion 'generate shell completion'
            cand -f 'use file'
            cand --file 'use file'
            cand -o 'origin language of the querying text'
            cand --lang-origin 'origin language of the querying text'
            cand -t 'the language to be translated into'
            cand --lang-target 'the language to be translated into'
            cand --format 'response format'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand --server 'server mode'
            cand --standalone 'standalone client mode'
            cand --lang-code 'display all available language codes'
        }
    ]
    $completions[$command]
}
