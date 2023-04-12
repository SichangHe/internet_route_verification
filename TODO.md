# TODO

- [ ] Parse policies into data structures.
    - [ ] Parse `import` from `RPSLTestField` to specific data structure.
    - [ ] Parse `mp-import` from `RPSLTestField` to specific data structure.
    - [ ] Parse `export` from `RPSLTestField` to specific data structure.
    - [ ] Parse `default` from `RPSLTestField` to specific data structure.
    - [ ] Parse `mp-default` from `RPSLTestField` to specific data structure.

    Grammar from <https://www.rfc-editor.org/rfc/rfc4012#section-2.5>:

    ```
    mp-import  [protocol <protocol-1>] [into <protocol-2>]   optional,
              [afi <afi-list>]                              multi-valued
              from <mp-peering-1> [action <action-1>; ... <action-N>;]
              . . .
              from <mp-peering-M> [action <action-1>; ... <action-N>;]
              accept <mp-filter> [;]

    mp-export  [protocol <protocol-1>] [into <protocol-2>]   optional,
              [afi <afi-list>]                              multi-valued
              to <mp-peering-1> [action <action-1>; ... <action-N>;]
              . . .
              to <mp-peering-M> [action <action-1>; ... <action-N>;]
              announce <mp-filter> [;]

    mp-default [afi <afi-list>] to <mp-peering>              optional,
              [action <action-1>; ... <action-N>;]          multi-valued
              [networks <mp-filter>]
    ```
