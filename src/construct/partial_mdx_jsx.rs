//! MDX JSX occurs in [MDX JSX (flow)][mdx_jsx_flow] and
//! [MDX JSX (text)][mdx_jsx_text].
//!
//! ## Grammar
//!
//! MDX JSX forms with the following BNF
//! (<small>see [construct][crate::construct] for character groups</small>):
//!
//! ```bnf
//! ; constraint: markdown whitespace (`space_or_tab | eol`) is NOT
//! ; allowed directly after `<` in order to allow `1 < 3` in markdown.
//! mdx_jsx ::=
//!   '<' [closing]
//!   [*whitespace name [attributes_after_identifier] [closing]]
//!   *whitespace '>'
//!
//! attributes_after_identifier ::=
//!   1*whitespace (attributes_boolean | attributes_value) |
//!   *whitespace attributes_expression |
//! attributes_after_value ::=
//!   *whitespace (attributes_boolean | attributes_expression | attributes_value)
//! attributes_boolean ::= key [attributes_after_identifier]
//! ; Note: in gnostic mode the value of the expression must instead be a single valid ES spread
//! ; expression
//! attributes_expression ::= expression [attributes_after_value]
//! attributes_value ::= key initializer [attributes_after_value]
//!
//! closing ::= *whitespace '/'
//!
//! name ::= identifier [local | members]
//! key ::= identifier [local]
//! local ::= *whitespace ':' *whitespace identifier
//! members ::= member *member
//! member ::= *whitespace '.' *whitespace identifier
//!
//! identifier ::= identifier_start *identifier_part
//! initializer ::= *whitespace '=' *whitespace value
//! value ::= double_quoted | single_quoted | expression
//! ; Note: in gnostic mode the value must instead be a single valid ES expression
//! expression ::= '{' *(expression_text | expression) '}'
//!
//! double_quoted ::= '"' *double_quoted_text '"'
//! single_quoted ::= "'" *single_quoted_text "'"
//!
//! text ::= char - '<' - '{'
//! whitespace ::= es_whitespace
//! double_quoted_text ::= char - '"'
//! single_quoted_text ::= char - "'"
//! expression_text ::= char - '{' - '}'
//! identifier_start ::= es_identifier_start
//! identifier_part ::= es_identifier_part | '-'
//!
//! ; ECMAScript
//! ; See “Identifier_start”: <https://tc39.es/ecma262/#prod-IdentifierStart>
//! es_identifier_start ::= ?
//! ; See “Identifier_part”: <https://tc39.es/ecma262/#prod-IdentifierPart>
//! es_identifier_part ::= ?
//! ; See “Whitespace”: <https://tc39.es/ecma262/#prod-WhiteSpace>
//! es_whitespace ::= ?
//! ```
//!
//! The grammar for JSX in markdown is much stricter than that of HTML in
//! markdown.
//! The primary benefit of this is that tags are parsed into tokens, and thus
//! can be processed.
//! Another, arguable, benefit of this is that it comes with syntax errors: if
//! an author types something that is nonsensical, an error is thrown with
//! information about where it happened, what occurred, and what was expected
//! instead.
//!
//! ## Tokens
//!
//! *   [`LineEnding`][Name::LineEnding]
//! *   [`MdxJsxEsWhitespace`][Name::MdxJsxEsWhitespace]
//! *   [`MdxJsxTagMarker`][Name::MdxJsxTagMarker]
//! *   [`MdxJsxTagClosingMarker`][Name::MdxJsxTagClosingMarker]
//! *   [`MdxJsxTagName`][Name::MdxJsxTagName]
//! *   [`MdxJsxTagNamePrimary`][Name::MdxJsxTagNamePrimary]
//! *   [`MdxJsxTagNameMemberMarker`][Name::MdxJsxTagNameMemberMarker]
//! *   [`MdxJsxTagNamePrefixMarker`][Name::MdxJsxTagNamePrefixMarker]
//! *   [`MdxJsxTagNameMember`][Name::MdxJsxTagNameMember]
//! *   [`MdxJsxTagNameLocal`][Name::MdxJsxTagNameLocal]
//! *   [`MdxJsxTagAttribute`][Name::MdxJsxTagAttribute]
//! *   [`MdxJsxTagAttributeName`][Name::MdxJsxTagAttributeName]
//! *   [`MdxJsxTagAttributePrimaryName`][Name::MdxJsxTagAttributePrimaryName]
//! *   [`MdxJsxTagAttributeNamePrefixMarker`][Name::MdxJsxTagAttributeNamePrefixMarker]
//! *   [`MdxJsxTagAttributeNameLocal`][Name::MdxJsxTagAttributeNameLocal]
//! *   [`MdxJsxTagAttributeInitializerMarker`][Name::MdxJsxTagAttributeInitializerMarker]
//! *   [`MdxJsxTagAttributeValueLiteral`][Name::MdxJsxTagAttributeValueLiteral]
//! *   [`MdxJsxTagAttributeValueLiteralMarker`][Name::MdxJsxTagAttributeValueLiteralMarker]
//! *   [`MdxJsxTagAttributeValueLiteralValue`][Name::MdxJsxTagAttributeValueLiteralValue]
//! *   [`MdxJsxTagSelfClosingMarker`][Name::MdxJsxTagSelfClosingMarker]
//!
//! ## Recommendation
//!
//! When authoring markdown with JSX, keep in mind that MDX is a whitespace
//! sensitive and line-based language, while JavaScript is insensitive to
//! whitespace.
//! This affects how markdown and JSX interleave with eachother in MDX.
//! For more info on how it works, see [§ Interleaving][interleaving] on the
//! MDX site.
//!
//! ###### Comments inside tags
//!
//! JavaScript comments in JSX are not supported.
//!
//! Incorrect:
//!
//! ```jsx
//! <hi/*comment!*//>
//! <hello// comment!
//! />
//! ```
//!
//! Correct:
//!
//! ```jsx
//! <hi/>
//! <hello
//! />
//! ```
//!
//! A PR that adds support for them would be accepted.
//!
//! ###### Element or fragment attribute values
//!
//! JSX elements or JSX fragments as attribute values are not supported.
//! The reason for this change is that it would be confusing whether markdown
//! would work.
//!
//! Incorrect:
//!
//! ```jsx
//! <welcome name=<>Venus</> />
//! <welcome name=<span>Pluto</span> />
//! ```
//!
//! Correct:
//!
//! ```jsx
//! <welcome name='Mars' />
//! <welcome name={<span>Jupiter</span>} />
//! ```
//!
//! ###### Greater than (`>`) and right curly brace (`}`)
//!
//! JSX does not allow U+003E GREATER THAN (`>`) or U+007D RIGHT CURLY BRACE
//! (`}`) literally in text, they need to be encoded as character references
//! (or expressions).
//! There is no good reason for this (some JSX parsers agree with us and don’t
//! crash either).
//! Therefore, in MDX, U+003E GREATER THAN (`>`) and U+007D RIGHT CURLY BRACE
//! (`}`) are fine literally and don’t need to be encoded.
//!
//! ## References
//!
//! *   [`jsx-flow.js` in `micromark-extension-mdx-jsx`](https://github.com/micromark/micromark-extension-mdx-jsx/blob/main/dev/lib/jsx-flow.js)
//! *   [`mdxjs.com`](https://mdxjs.com)
//!
//! [mdx_jsx_flow]: crate::construct::mdx_jsx_flow
//! [mdx_jsx_text]: crate::construct::mdx_jsx_text
//! [interleaving]: https://mdxjs.com/docs/what-is-mdx/#interleaving

use crate::event::Name;
use crate::state::{Name as StateName, State};
use crate::tokenizer::Tokenizer;
use crate::util::{
    classify_character::Kind as CharacterKind,
    slice::{byte_to_kind, char_after_index},
};
use alloc::{
    format,
    string::{String, ToString},
};
use core::str;
use unicode_id::UnicodeID;

/// Start of MDX: JSX.
///
/// ```markdown
/// > | a <B /> c
///       ^
/// ```
pub fn start(tokenizer: &mut Tokenizer) -> State {
    debug_assert_eq!(tokenizer.current, Some(b'<'));
    tokenizer.enter(tokenizer.tokenize_state.token_1.clone());
    tokenizer.enter(Name::MdxJsxTagMarker);
    tokenizer.consume();
    tokenizer.exit(Name::MdxJsxTagMarker);
    State::Next(StateName::MdxJsxStartAfter)
}

/// After `<`.
///
/// ```markdown
/// > | a <B /> c
///        ^
/// ```
pub fn start_after(tokenizer: &mut Tokenizer) -> State {
    // Deviate from JSX, which allows arbitrary whitespace.
    // See: <https://github.com/micromark/micromark-extension-mdx-jsx/issues/7>.
    if let Some(b'\t' | b'\n' | b' ') = tokenizer.current {
        State::Nok
    } else {
        tokenizer.attempt(State::Next(StateName::MdxJsxNameBefore), State::Nok);
        State::Retry(StateName::MdxJsxEsWhitespaceStart)
    }
}

/// Before name, self slash, or end of tag for fragments.
///
/// ```markdown
/// > | a <B> c
///        ^
/// > | a </B> c
///        ^
/// > | a <> b
///        ^
/// ```
pub fn name_before(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        // Closing tag.
        Some(b'/') => {
            tokenizer.enter(Name::MdxJsxTagClosingMarker);
            tokenizer.consume();
            tokenizer.exit(Name::MdxJsxTagClosingMarker);
            tokenizer.attempt(
                State::Next(StateName::MdxJsxClosingTagNameBefore),
                State::Nok,
            );
            State::Next(StateName::MdxJsxEsWhitespaceStart)
        }
        // Fragment opening tag.
        Some(b'>') => State::Retry(StateName::MdxJsxTagEnd),
        _ => {
            if id_start(char_after_index(
                tokenizer.parse_state.bytes,
                tokenizer.point.index,
            )) {
                tokenizer.enter(Name::MdxJsxTagName);
                tokenizer.enter(Name::MdxJsxTagNamePrimary);
                tokenizer.consume();
                State::Next(StateName::MdxJsxPrimaryName)
            } else {
                crash(
                    tokenizer,
                    "before name",
                    &format!(
                        "a character that can start a name, such as a letter, `$`, or `_`{}",
                        if tokenizer.current == Some(b'!') {
                            " (note: to create a comment in MDX, use `{/* text */}`)"
                        } else {
                            ""
                        }
                    ),
                )
            }
        }
    }
}

/// Before name of closing tag or end of closing fragment tag.
///
/// ```markdown
/// > | a </> b
///         ^
/// > | a </B> c
///         ^
/// ```
pub fn closing_tag_name_before(tokenizer: &mut Tokenizer) -> State {
    // Fragment closing tag.
    if let Some(b'>') = tokenizer.current {
        State::Retry(StateName::MdxJsxTagEnd)
    }
    // Start of a closing tag name.
    else if id_start(char_after_index(
        tokenizer.parse_state.bytes,
        tokenizer.point.index,
    )) {
        tokenizer.enter(Name::MdxJsxTagName);
        tokenizer.enter(Name::MdxJsxTagNamePrimary);
        tokenizer.consume();
        State::Next(StateName::MdxJsxPrimaryName)
    } else {
        crash(
            tokenizer,
            "before name",
            &format!(
                "a character that can start a name, such as a letter, `$`, or `_`{}",
                if tokenizer.current == Some(b'*' | b'/') {
                    " (note: JS comments in JSX tags are not supported in MDX)"
                } else {
                    ""
                }
            ),
        )
    }
}

/// In primary name.
///
/// ```markdown
/// > | a <Bc> d
///         ^
/// ```
pub fn primary_name(tokenizer: &mut Tokenizer) -> State {
    // End of name.
    if byte_to_kind(tokenizer.parse_state.bytes, tokenizer.point.index) == CharacterKind::Whitespace
        || matches!(tokenizer.current, Some(b'.' | b'/' | b':' | b'>' | b'{'))
    {
        tokenizer.exit(Name::MdxJsxTagNamePrimary);
        tokenizer.attempt(State::Next(StateName::MdxJsxPrimaryNameAfter), State::Nok);
        State::Retry(StateName::MdxJsxEsWhitespaceStart)
    }
    // Continuation of name: remain.
    // Allow continuation bytes.
    else if matches!(tokenizer.current, Some(0x80..=0xBF))
        || id_cont(char_after_index(
            tokenizer.parse_state.bytes,
            tokenizer.point.index,
        ))
    {
        tokenizer.consume();
        State::Next(StateName::MdxJsxPrimaryName)
    } else {
        crash(
            tokenizer,
            "in name",
            &format!(
                "a name character such as letters, digits, `$`, or `_`; whitespace before attributes; or the end of the tag{}",
                if tokenizer.current == Some(b'@') {
                    " (note: to create a link in MDX, use `[text](url)`)"
                } else {
                    ""
                }
            ),
        )
    }
}

/// After primary name.
///
/// ```markdown
/// > | a <b.c> d
///         ^
/// > | a <b:c> d
///         ^
/// ```
pub fn primary_name_after(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        // Start of a member name.
        Some(b'.') => {
            tokenizer.enter(Name::MdxJsxTagNameMemberMarker);
            tokenizer.consume();
            tokenizer.exit(Name::MdxJsxTagNameMemberMarker);
            tokenizer.attempt(State::Next(StateName::MdxJsxMemberNameBefore), State::Nok);
            State::Next(StateName::MdxJsxEsWhitespaceStart)
        }
        // Start of a local name.
        Some(b':') => {
            tokenizer.enter(Name::MdxJsxTagNamePrefixMarker);
            tokenizer.consume();
            tokenizer.exit(Name::MdxJsxTagNamePrefixMarker);
            tokenizer.attempt(State::Next(StateName::MdxJsxLocalNameBefore), State::Nok);
            State::Next(StateName::MdxJsxEsWhitespaceStart)
        }
        // End of name.
        _ => {
            if matches!(tokenizer.current, Some(b'/' | b'>' | b'{'))
                || id_start(char_after_index(
                    tokenizer.parse_state.bytes,
                    tokenizer.point.index,
                ))
            {
                tokenizer.exit(Name::MdxJsxTagName);
                State::Retry(StateName::MdxJsxAttributeBefore)
            } else {
                crash(
                    tokenizer,
                    "after name",
                    "a character that can start an attribute name, such as a letter, `$`, or `_`; whitespace before attributes; or the end of the tag"
                )
            }
        }
    }
}

/// Before member name.
///
/// ```markdown
/// > | a <b.c> d
///          ^
/// ```
pub fn member_name_before(tokenizer: &mut Tokenizer) -> State {
    // Start of a member name.
    if id_start(char_after_index(
        tokenizer.parse_state.bytes,
        tokenizer.point.index,
    )) {
        tokenizer.enter(Name::MdxJsxTagNameMember);
        tokenizer.consume();
        State::Next(StateName::MdxJsxMemberName)
    } else {
        crash(
            tokenizer,
            "before member name",
            "a character that can start an attribute name, such as a letter, `$`, or `_`; whitespace before attributes; or the end of the tag"
        )
    }
}

/// In member name.
///
/// ```markdown
/// > | a <b.cd> e
///           ^
/// ```
pub fn member_name(tokenizer: &mut Tokenizer) -> State {
    // End of name.
    // Note: no `:` allowed here.
    if byte_to_kind(tokenizer.parse_state.bytes, tokenizer.point.index) == CharacterKind::Whitespace
        || matches!(tokenizer.current, Some(b'.' | b'/' | b'>' | b'{'))
    {
        tokenizer.exit(Name::MdxJsxTagNameMember);
        tokenizer.attempt(State::Next(StateName::MdxJsxMemberNameAfter), State::Nok);
        State::Retry(StateName::MdxJsxEsWhitespaceStart)
    }
    // Continuation of name: remain.
    // Allow continuation bytes.
    else if matches!(tokenizer.current, Some(0x80..=0xBF))
        || id_cont(char_after_index(
            tokenizer.parse_state.bytes,
            tokenizer.point.index,
        ))
    {
        tokenizer.consume();
        State::Next(StateName::MdxJsxMemberName)
    } else {
        crash(
            tokenizer,
            "in member name",
            &format!(
                "a name character such as letters, digits, `$`, or `_`; whitespace before attributes; or the end of the tag{}",
                if tokenizer.current == Some(b'@') {
                    " (note: to create a link in MDX, use `[text](url)`)"
                } else {
                    ""
                }
            ),
        )
    }
}

/// After member name.
///
/// ```markdown
/// > | a <b.c> d
///           ^
/// > | a <b.c.d> e
///           ^
/// ```
pub fn member_name_after(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        // Start of another member name.
        Some(b'.') => {
            tokenizer.enter(Name::MdxJsxTagNameMemberMarker);
            tokenizer.consume();
            tokenizer.exit(Name::MdxJsxTagNameMemberMarker);
            tokenizer.attempt(State::Next(StateName::MdxJsxMemberNameBefore), State::Nok);
            State::Next(StateName::MdxJsxEsWhitespaceStart)
        }
        // End of name.
        _ => {
            if matches!(tokenizer.current, Some(b'/' | b'>' | b'{'))
                || id_start(char_after_index(
                    tokenizer.parse_state.bytes,
                    tokenizer.point.index,
                ))
            {
                tokenizer.exit(Name::MdxJsxTagName);
                State::Retry(StateName::MdxJsxAttributeBefore)
            } else {
                crash(
                    tokenizer,
                    "after member name",
                    "a character that can start an attribute name, such as a letter, `$`, or `_`; whitespace before attributes; or the end of the tag"
                )
            }
        }
    }
}

/// Local member name.
///
/// ```markdown
/// > | a <b:c> d
///          ^
/// ```
pub fn local_name_before(tokenizer: &mut Tokenizer) -> State {
    // Start of a local name.
    if id_start(char_after_index(
        tokenizer.parse_state.bytes,
        tokenizer.point.index,
    )) {
        tokenizer.enter(Name::MdxJsxTagNameLocal);
        tokenizer.consume();
        State::Next(StateName::MdxJsxLocalName)
    } else {
        crash(
            tokenizer,
            "before local name",
            &format!(
                "a character that can start a name, such as a letter, `$`, or `_`{}",
                if matches!(tokenizer.current, Some(b'+' | b'/'..=b'9')) {
                    " (note: to create a link in MDX, use `[text](url)`)"
                } else {
                    ""
                }
            ),
        )
    }
}

/// In local name.
///
/// ```markdown
/// > | a <b:cd> e
///           ^
/// ```
pub fn local_name(tokenizer: &mut Tokenizer) -> State {
    // End of local name (note that we don’t expect another colon, or a member).
    if byte_to_kind(tokenizer.parse_state.bytes, tokenizer.point.index) == CharacterKind::Whitespace
        || matches!(tokenizer.current, Some(b'/' | b'>' | b'{'))
    {
        tokenizer.exit(Name::MdxJsxTagNameLocal);
        tokenizer.attempt(State::Next(StateName::MdxJsxLocalNameAfter), State::Nok);
        State::Retry(StateName::MdxJsxEsWhitespaceStart)
    }
    // Continuation of name: remain.
    // Allow continuation bytes.
    else if matches!(tokenizer.current, Some(0x80..=0xBF))
        || id_cont(char_after_index(
            tokenizer.parse_state.bytes,
            tokenizer.point.index,
        ))
    {
        tokenizer.consume();
        State::Next(StateName::MdxJsxLocalName)
    } else {
        crash(
            tokenizer,
            "in local name",
            "a name character such as letters, digits, `$`, or `_`; whitespace before attributes; or the end of the tag"
        )
    }
}

/// After local name.
///
/// This is like as `primary_name_after`, but we don’t expect colons or
/// periods.
///
/// ```markdown
/// > | a <b.c> d
///           ^
/// > | a <b.c.d> e
///           ^
/// ```
pub fn local_name_after(tokenizer: &mut Tokenizer) -> State {
    // End of name.
    if matches!(tokenizer.current, Some(b'/' | b'>' | b'{'))
        || id_start(char_after_index(
            tokenizer.parse_state.bytes,
            tokenizer.point.index,
        ))
    {
        tokenizer.exit(Name::MdxJsxTagName);
        State::Retry(StateName::MdxJsxAttributeBefore)
    } else {
        crash(
            tokenizer,
            "after local name",
            "a character that can start an attribute name, such as a letter, `$`, or `_`; whitespace before attributes; or the end of the tag"
        )
    }
}

/// Before attribute.
///
/// ```markdown
/// > | a <b /> c
///          ^
/// > | a <b > c
///          ^
/// > | a <b {...c}> d
///          ^
/// > | a <b c> d
///          ^
/// ```
pub fn attribute_before(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        // Self-closing.
        Some(b'/') => {
            tokenizer.enter(Name::MdxJsxTagSelfClosingMarker);
            tokenizer.consume();
            tokenizer.exit(Name::MdxJsxTagSelfClosingMarker);
            tokenizer.attempt(State::Next(StateName::MdxJsxSelfClosing), State::Nok);
            State::Next(StateName::MdxJsxEsWhitespaceStart)
        }
        // End of tag.
        Some(b'>') => State::Retry(StateName::MdxJsxTagEnd),
        // Attribute expression.
        Some(b'{') => unreachable!("to do: attribute expression"),
        _ => {
            // Start of an attribute name.
            if id_start(char_after_index(
                tokenizer.parse_state.bytes,
                tokenizer.point.index,
            )) {
                tokenizer.enter(Name::MdxJsxTagAttribute);
                tokenizer.enter(Name::MdxJsxTagAttributeName);
                tokenizer.enter(Name::MdxJsxTagAttributePrimaryName);
                tokenizer.consume();
                State::Next(StateName::MdxJsxAttributePrimaryName)
            } else {
                crash(
                    tokenizer,
                    "before attribute name",
                    "a character that can start an attribute name, such as a letter, `$`, or `_`; whitespace before attributes; or the end of the tag"
                )
            }
        }
    }
}

/// In primary attribute name.
///
/// ```markdown
/// > | a <b cd/> e
///           ^
/// > | a <b c:d> e
///           ^
/// > | a <b c=d> e
///           ^
/// ```
pub fn attribute_primary_name(tokenizer: &mut Tokenizer) -> State {
    // End of attribute name or tag.
    if byte_to_kind(tokenizer.parse_state.bytes, tokenizer.point.index) == CharacterKind::Whitespace
        || matches!(tokenizer.current, Some(b'/' | b':' | b'=' | b'>' | b'{'))
    {
        tokenizer.exit(Name::MdxJsxTagAttributePrimaryName);
        tokenizer.attempt(
            State::Next(StateName::MdxJsxAttributePrimaryNameAfter),
            State::Nok,
        );
        State::Retry(StateName::MdxJsxEsWhitespaceStart)
    }
    // Continuation of name: remain.
    // Allow continuation bytes.
    else if matches!(tokenizer.current, Some(0x80..=0xBF))
        || id_cont(char_after_index(
            tokenizer.parse_state.bytes,
            tokenizer.point.index,
        ))
    {
        tokenizer.consume();
        State::Next(StateName::MdxJsxAttributePrimaryName)
    } else {
        crash(
            tokenizer,
            "in attribute name",
            "an attribute name character such as letters, digits, `$`, or `_`; `=` to initialize a value; whitespace before attributes; or the end of the tag"
        )
    }
}

/// After primary attribute name.
///
/// ```markdown
/// > | a <b c/> d
///           ^
/// > | a <b c:d> e
///           ^
/// > | a <b c=d> e
///           ^
/// ```
pub fn attribute_primary_name_after(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        // Start of a local name.
        Some(b':') => {
            tokenizer.enter(Name::MdxJsxTagAttributeNamePrefixMarker);
            tokenizer.consume();
            tokenizer.exit(Name::MdxJsxTagAttributeNamePrefixMarker);
            tokenizer.attempt(
                State::Next(StateName::MdxJsxAttributeLocalNameBefore),
                State::Nok,
            );
            State::Next(StateName::MdxJsxEsWhitespaceStart)
        }
        // Initializer: start of an attribute value.
        Some(b'=') => {
            tokenizer.exit(Name::MdxJsxTagAttributeName);
            tokenizer.enter(Name::MdxJsxTagAttributeInitializerMarker);
            tokenizer.consume();
            tokenizer.exit(Name::MdxJsxTagAttributeInitializerMarker);
            tokenizer.attempt(
                State::Next(StateName::MdxJsxAttributeValueBefore),
                State::Nok,
            );
            State::Retry(StateName::MdxJsxEsWhitespaceStart)
        }
        _ => {
            // End of tag / new attribute.
            if byte_to_kind(tokenizer.parse_state.bytes, tokenizer.point.index)
                == CharacterKind::Whitespace
                || matches!(tokenizer.current, Some(b'/' | b'>' | b'{'))
                || id_start(char_after_index(
                    tokenizer.parse_state.bytes,
                    tokenizer.point.index,
                ))
            {
                tokenizer.exit(Name::MdxJsxTagAttributeName);
                tokenizer.exit(Name::MdxJsxTagAttribute);
                tokenizer.attempt(State::Next(StateName::MdxJsxAttributeBefore), State::Nok);
                State::Retry(StateName::MdxJsxEsWhitespaceStart)
            } else {
                crash(
                    tokenizer,
                    "after attribute name",
                    "a character that can start an attribute name, such as a letter, `$`, or `_`; `=` to initialize a value; or the end of the tag"
                )
            }
        }
    }
}

/// Before local attribute name.
///
/// ```markdown
/// > | a <b c:d/> e
///            ^
/// ```
pub fn attribute_local_name_before(tokenizer: &mut Tokenizer) -> State {
    // Start of a local name.
    if id_start(char_after_index(
        tokenizer.parse_state.bytes,
        tokenizer.point.index,
    )) {
        tokenizer.enter(Name::MdxJsxTagAttributeNameLocal);
        tokenizer.consume();
        State::Next(StateName::MdxJsxAttributeLocalName)
    } else {
        crash(
            tokenizer,
            "before local attribute name",
            "a character that can start an attribute name, such as a letter, `$`, or `_`; `=` to initialize a value; or the end of the tag"
        )
    }
}

/// In local attribute name.
///
/// ```markdown
/// > | a <b c:de/> f
///             ^
/// > | a <b c:d=e/> f
///             ^
/// ```
pub fn attribute_local_name(tokenizer: &mut Tokenizer) -> State {
    // End of local name (note that we don’t expect another colon).
    if byte_to_kind(tokenizer.parse_state.bytes, tokenizer.point.index) == CharacterKind::Whitespace
        || matches!(tokenizer.current, Some(b'/' | b'=' | b'>' | b'{'))
    {
        tokenizer.exit(Name::MdxJsxTagAttributeNameLocal);
        tokenizer.exit(Name::MdxJsxTagAttributeName);
        tokenizer.attempt(
            State::Next(StateName::MdxJsxAttributeLocalNameAfter),
            State::Nok,
        );
        State::Retry(StateName::MdxJsxEsWhitespaceStart)
    }
    // Continuation of name: remain.
    // Allow continuation bytes.
    else if matches!(tokenizer.current, Some(0x80..=0xBF))
        || id_cont(char_after_index(
            tokenizer.parse_state.bytes,
            tokenizer.point.index,
        ))
    {
        tokenizer.consume();
        State::Next(StateName::MdxJsxAttributeLocalName)
    } else {
        crash(
            tokenizer,
            "in local attribute name",
            "an attribute name character such as letters, digits, `$`, or `_`; `=` to initialize a value; whitespace before attributes; or the end of the tag"
        )
    }
}

/// After local attribute name.
///
/// ```markdown
/// > | a <b c:d/> f
///             ^
/// > | a <b c:d=e/> f
///             ^
/// ```
pub fn attribute_local_name_after(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        // Start of an attribute value.
        Some(b'=') => {
            tokenizer.enter(Name::MdxJsxTagAttributeInitializerMarker);
            tokenizer.consume();
            tokenizer.exit(Name::MdxJsxTagAttributeInitializerMarker);
            tokenizer.attempt(
                State::Next(StateName::MdxJsxAttributeValueBefore),
                State::Nok,
            );
            State::Next(StateName::MdxJsxEsWhitespaceStart)
        }
        _ => {
            // End of name.
            if matches!(tokenizer.current, Some(b'/' | b'>' | b'{'))
                || id_start(char_after_index(
                    tokenizer.parse_state.bytes,
                    tokenizer.point.index,
                ))
            {
                tokenizer.exit(Name::MdxJsxTagAttribute);
                State::Retry(StateName::MdxJsxAttributeBefore)
            } else {
                crash(
                    tokenizer,
                    "after local attribute name",
                    "a character that can start an attribute name, such as a letter, `$`, or `_`; `=` to initialize a value; or the end of the tag"
                )
            }
        }
    }
}

/// After `=`, before value.
///
/// ```markdown
/// > | a <b c="d"/> e
///            ^
/// > | a <b c={d}/> e
///            ^
/// ```
pub fn attribute_value_before(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        // Start of double- or single quoted value.
        Some(b'"' | b'\'') => {
            tokenizer.tokenize_state.marker = tokenizer.current.unwrap();
            tokenizer.enter(Name::MdxJsxTagAttributeValueLiteral);
            tokenizer.enter(Name::MdxJsxTagAttributeValueLiteralMarker);
            tokenizer.consume();
            tokenizer.exit(Name::MdxJsxTagAttributeValueLiteralMarker);
            State::Next(StateName::MdxJsxAttributeValueQuotedStart)
        }
        // Attribute value expression.
        Some(b'{') => unreachable!("to do: attribute value expression"),
        _ => crash(
            tokenizer,
            "before attribute value",
            &format!(
                "a character that can start an attribute value, such as `\"`, `'`, or `{{`{}",
                if tokenizer.current == Some(b'<') {
                    " (note: to use an element or fragment as a prop value in MDX, use `{<element />}`)"
                } else {
                    ""
                }
            ),
        ),
    }
}

/// Before quoted literal attribute value.
///
/// ```markdown
/// > | a <b c="d"/> e
///            ^
/// ```
pub fn attribute_value_quoted_start(tokenizer: &mut Tokenizer) -> State {
    if let Some(byte) = tokenizer.current {
        if byte == tokenizer.tokenize_state.marker {
            tokenizer.tokenize_state.marker = 0;
            tokenizer.enter(Name::MdxJsxTagAttributeValueLiteralMarker);
            tokenizer.consume();
            tokenizer.exit(Name::MdxJsxTagAttributeValueLiteralMarker);
            tokenizer.exit(Name::MdxJsxTagAttributeValueLiteral);
            tokenizer.exit(Name::MdxJsxTagAttribute);
            tokenizer.attempt(State::Next(StateName::MdxJsxAttributeBefore), State::Nok);
            State::Next(StateName::MdxJsxEsWhitespaceStart)
        } else if byte == b'\n' {
            tokenizer.attempt(
                State::Next(StateName::MdxJsxAttributeValueQuotedStart),
                State::Nok,
            );
            State::Retry(StateName::MdxJsxEsWhitespaceStart)
        } else {
            tokenizer.enter(Name::MdxJsxTagAttributeValueLiteralValue);
            State::Retry(StateName::MdxJsxAttributeValueQuoted)
        }
    } else {
        crash(
            tokenizer,
            "in attribute value",
            &format!(
                "a corresponding closing quote {}",
                format_byte(tokenizer.tokenize_state.marker)
            ),
        )
    }
}

/// In quoted literal attribute value.
///
/// ```markdown
/// > | a <b c="d"/> e
///             ^
/// ```
pub fn attribute_value_quoted(tokenizer: &mut Tokenizer) -> State {
    if tokenizer.current == Some(tokenizer.tokenize_state.marker)
        || matches!(tokenizer.current, None | Some(b'\n'))
    {
        tokenizer.exit(Name::MdxJsxTagAttributeValueLiteralValue);
        State::Retry(StateName::MdxJsxAttributeValueQuotedStart)
    } else {
        tokenizer.consume();
        State::Next(StateName::MdxJsxAttributeValueQuoted)
    }
}

/// After self-closing slash.
///
/// ```markdown
/// > | a <b/> c
///          ^
/// ```
pub fn self_closing(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'>') => State::Retry(StateName::MdxJsxTagEnd),
        _ => crash(
            tokenizer,
            "after self-closing slash",
            &format!(
                "`>` to end the tag{}",
                if tokenizer.current == Some(b'*' | b'/') {
                    " (note: JS comments in JSX tags are not supported in MDX)"
                } else {
                    ""
                }
            ),
        ),
    }
}

/// At final `>`.
///
/// ```markdown
/// > | a <b> c
///         ^
/// ```
pub fn tag_end(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'>') => {
            tokenizer.enter(Name::MdxJsxTagMarker);
            tokenizer.consume();
            tokenizer.exit(Name::MdxJsxTagMarker);
            tokenizer.exit(tokenizer.tokenize_state.token_1.clone());
            State::Ok
        }
        _ => unreachable!("expected `>`"),
    }
}

/// Before optional ECMAScript whitespace.
///
/// ```markdown
/// > | a <a b> c
///         ^
/// ```
pub fn es_whitespace_start(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'\n') => State::Retry(StateName::MdxJsxEsWhitespaceEol),
        _ => {
            if byte_to_kind(tokenizer.parse_state.bytes, tokenizer.point.index)
                == CharacterKind::Whitespace
            {
                tokenizer.enter(Name::MdxJsxEsWhitespace);
                State::Retry(StateName::MdxJsxEsWhitespaceInside)
            } else {
                State::Ok
            }
        }
    }
}

/// In ECMAScript whitespace.
///
/// ```markdown
/// > | a <a  b> c
///          ^
/// ```
pub fn es_whitespace_inside(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'\n') => {
            tokenizer.exit(Name::MdxJsxEsWhitespace);
            State::Retry(StateName::MdxJsxEsWhitespaceEol)
        }
        // Allow continuation bytes.
        Some(0x80..=0xBF) => {
            tokenizer.consume();
            State::Next(StateName::MdxJsxEsWhitespaceInside)
        }
        _ => {
            if byte_to_kind(tokenizer.parse_state.bytes, tokenizer.point.index)
                == CharacterKind::Whitespace
            {
                tokenizer.consume();
                State::Next(StateName::MdxJsxEsWhitespaceInside)
            } else {
                tokenizer.exit(Name::MdxJsxEsWhitespace);
                State::Ok
            }
        }
    }
}

pub fn es_whitespace_eol(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'\n') => {
            tokenizer.enter(Name::LineEnding);
            tokenizer.consume();
            tokenizer.exit(Name::LineEnding);
            State::Next(StateName::MdxJsxEsWhitespaceEolAfter)
        }
        _ => State::Ok,
    }
}

pub fn es_whitespace_eol_after(tokenizer: &mut Tokenizer) -> State {
    if tokenizer.tokenize_state.token_1 == Name::MdxJsxFlowTag && tokenizer.lazy {
        crash_lazy(tokenizer)
    } else if byte_to_kind(tokenizer.parse_state.bytes, tokenizer.point.index)
        == CharacterKind::Whitespace
    {
        tokenizer.enter(Name::MdxJsxEsWhitespace);
        State::Retry(StateName::MdxJsxEsWhitespaceEolAfterInside)
    } else {
        State::Ok
    }
}

pub fn es_whitespace_eol_after_inside(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        // Not allowed.
        Some(b'\n') => State::Nok,
        // Allow continuation bytes.
        Some(0x80..=0xBF) => {
            tokenizer.consume();
            State::Next(StateName::MdxJsxEsWhitespaceEolAfterInside)
        }
        _ => {
            if byte_to_kind(tokenizer.parse_state.bytes, tokenizer.point.index)
                == CharacterKind::Whitespace
            {
                tokenizer.consume();
                State::Next(StateName::MdxJsxEsWhitespaceEolAfterInside)
            } else {
                tokenizer.exit(Name::MdxJsxEsWhitespace);
                State::Ok
            }
        }
    }
}

fn id_start(code: Option<char>) -> bool {
    if let Some(char) = code {
        UnicodeID::is_id_start(char) || matches!(char, '$' | '_')
    } else {
        false
    }
}

fn id_cont(code: Option<char>) -> bool {
    if let Some(char) = code {
        UnicodeID::is_id_continue(char) || matches!(char, '-' | '\u{200c}' | '\u{200d}')
    } else {
        false
    }
}

fn crash_lazy(tokenizer: &Tokenizer) -> State {
    State::Error(format!(
        "{}:{}: Unexpected lazy line in container, expected line to be prefixed with `>` when in a block quote, whitespace when in a list, etc",
        tokenizer.point.line, tokenizer.point.column
    ))
}

fn crash(tokenizer: &Tokenizer, at: &str, expect: &str) -> State {
    let char = if tokenizer.current == None {
        None
    } else {
        char_after_index(tokenizer.parse_state.bytes, tokenizer.point.index)
    };

    // To do: externalize this, and the print mechanism in the tokenizer,
    // to one proper formatter.
    let actual = match char {
        None => "end of file".to_string(),
        Some(char) => format!("character {}", format_char(char)),
    };

    State::Error(format!(
        "{}:{}: Unexpected {} {}, expected {}",
        tokenizer.point.line, tokenizer.point.column, actual, at, expect
    ))
}

fn format_char(char: char) -> String {
    let unicode = format!("U+{:>04X}", char as u32);
    let printable = match char {
        '`' => Some("`` ` ``".to_string()),
        ' '..='~' => Some(format!("`{}`", char)),
        _ => None,
    };

    if let Some(char) = printable {
        format!("{} ({})", char, unicode)
    } else {
        unicode
    }
}

fn format_byte(byte: u8) -> String {
    let unicode = format!("U+{:>04X}", byte);
    let printable = match byte {
        b'`' => Some("`` ` ``".to_string()),
        b' '..=b'~' => Some(format!("`{}`", str::from_utf8(&[byte]).unwrap())),
        _ => None,
    };

    if let Some(char) = printable {
        format!("{} ({})", char, unicode)
    } else {
        unicode
    }
}
