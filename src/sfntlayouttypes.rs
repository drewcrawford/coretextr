/**I am not 100% sure what the size of this enum is.  It is definitely signed.*/
pub struct FeatureTypes(pub i8);
impl FeatureTypes {
    pub const ALL_TYPOGRAPHIC_FEATURES: FeatureTypes = FeatureTypes(0);
    pub const LIGATURES: FeatureTypes = FeatureTypes(1);
    pub const CURISVE_CONNECTION: FeatureTypes = FeatureTypes(2);
    pub const LETTER_CASE: FeatureTypes = FeatureTypes(3);
    pub const VERTICAL_SUBSTITUTION: FeatureTypes = FeatureTypes(4);
    pub const LINGUISTIC_REARRANGEMENT: FeatureTypes = FeatureTypes(5);
    pub const NUMBER_SPACING: FeatureTypes = FeatureTypes(6);
    pub const SMART_SWASH_TYPE: FeatureTypes = FeatureTypes(8);
    pub const DIACRITICS_TYPE: FeatureTypes = FeatureTypes(9);
    pub const VERTICAL_POSITION: FeatureTypes = FeatureTypes(10);
    pub const FRACTIONS: FeatureTypes = FeatureTypes(11);
    pub const OVERLAPPING_CHARACTERS_TYPE: FeatureTypes = FeatureTypes(13);
    pub const TYPGRAPHIC_EXTRAS: FeatureTypes = FeatureTypes(14);
    pub const MATHEMATICAL_EXTRAS: FeatureTypes = FeatureTypes(15);
    pub const ORNAMENT_SETS: FeatureTypes = FeatureTypes(16);
    pub const CHARACTER_ALTERNATIVES: FeatureTypes = FeatureTypes(17);
    pub const DESIGN_COMPLEXITY: FeatureTypes = FeatureTypes(18);
    pub const STYLE_OPTIONS: FeatureTypes = FeatureTypes(19);
    pub const CHARACTER_SHAPE: FeatureTypes = FeatureTypes(20);
    pub const NUMBER_CASE: FeatureTypes = FeatureTypes(21);
    pub const TEXT_SPACING: FeatureTypes = FeatureTypes(22);
    pub const TRANSLITERATION: FeatureTypes = FeatureTypes(23);
    pub const ANNOTATION: FeatureTypes = FeatureTypes(24);
    pub const KANA_SPACING: FeatureTypes = FeatureTypes(25);
    pub const IDEOGRAPHIC_SPACING: FeatureTypes = FeatureTypes(26);
    pub const UNICODE_DECOMPOSITION: FeatureTypes = FeatureTypes(27);
    pub const RUBY_KANA: FeatureTypes = FeatureTypes(28);
    pub const CJK_SYMBOL_ALTERNATIVES: FeatureTypes = FeatureTypes(29);
    pub const IDEOGRAPHIC_ALTERNATIVES: FeatureTypes = FeatureTypes(30);
    pub const CJK_VERTICAL_ROMAN_PLACEMENT: FeatureTypes = FeatureTypes(31);
    pub const ITALIC_CJK_ROMAN: FeatureTypes = FeatureTypes(32);
    pub const CASE_SENSITIVE_LAYOUT: FeatureTypes = FeatureTypes(33);
    pub const ALTERNATE_KANA: FeatureTypes = FeatureTypes(34);
    pub const STYLISTIC_ALTERNATIVES: FeatureTypes = FeatureTypes(35);
    pub const CONTEXTUAL_ALTERNATIVES: FeatureTypes = FeatureTypes(36);
    pub const LOWER_CASE: FeatureTypes = FeatureTypes(37);
    pub const UPPER_CASE: FeatureTypes = FeatureTypes(38);
    pub const LANGUAGE_TAG: FeatureTypes = FeatureTypes(39);
    pub const CJK_ROMAN_SPACING: FeatureTypes = FeatureTypes(103);
    pub const LAST_FEATURE: FeatureTypes = FeatureTypes(-1);
}

pub struct AllFeatures(pub i8);
impl AllFeatures {
    pub const ON: AllFeatures = AllFeatures(0);
    pub const OFF: AllFeatures = AllFeatures(1);
}

pub struct Ligatures(pub i8);
impl Ligatures {
    pub const REQUIRED_LIGATURES_ON: Ligatures = Ligatures(0);
    pub const REQUIRED_LIGATURES_OFF: Ligatures = Ligatures(1);
    pub const COMMON_LIGATURES_ON: Ligatures = Ligatures(2);
    pub const COMMON_LIGATURES_OFF: Ligatures = Ligatures(3);
    pub const RARE_LIGATURES_ON: Ligatures = Ligatures(4);
    pub const RARE_LIGATURES_OFF: Ligatures = Ligatures(5);
    pub const LOGOS_ON: Ligatures = Ligatures(6);
    pub const LOGOS_OFF: Ligatures = Ligatures(7);
    pub const REBUS_PICTURES_ON: Ligatures = Ligatures(8);
    pub const REBUS_PICTURES_OFF: Ligatures = Ligatures(9);
    pub const DIPHTHONG_LIGATURES_ON: Ligatures = Ligatures(10);
    pub const DIPHTHONG_LIGATURES_OFF: Ligatures = Ligatures(11);
    pub const SQUARED_LIGATURES_ON: Ligatures = Ligatures(12);
    pub const SQUARED_LIGATURES_OFF: Ligatures = Ligatures(13);
    pub const ABBREV_SQUARED_LIGATURES_ON: Ligatures = Ligatures(14);
    pub const ABBREV_SQUARED_LIGATURES_OFF: Ligatures = Ligatures(15);
    pub const SYMBOL_LIGATURES_ON: Ligatures = Ligatures(16);
    pub const SYMBOL_LIGATURES_OFF: Ligatures = Ligatures(17);
    pub const CONTEXTUAL_LIGATURES_ON: Ligatures = Ligatures(18);
    pub const CONTEXTUAL_LIGATURES_OFF: Ligatures = Ligatures(19);
    pub const HISTORICAL_LIGATURES_ON: Ligatures = Ligatures(20);
    pub const HISTORICAL_LIGATURES_OFF: Ligatures = Ligatures(21);
}

pub struct CursiveConnection(pub i8);
impl CursiveConnection {
    pub const UNCONNECTED: CursiveConnection = CursiveConnection(0);
    pub const PARTIALLY_CONNECTED: CursiveConnection = CursiveConnection(1);
    pub const CURSIVE: CursiveConnection = CursiveConnection(2);
}

pub struct LetterCase(pub i8);
impl LetterCase {
    pub const UPPER_AND_LOWER_CASE: LetterCase = LetterCase(0);
    pub const ALL_CAPS: LetterCase = LetterCase(1);
    pub const ALL_LOWER_CASE: LetterCase = LetterCase(2);
    pub const SMALL_CAPS: LetterCase = LetterCase(3);
    pub const INITIAL_CAPS: LetterCase = LetterCase(4);
    pub const INITIAL_CAPS_AND_SMALL_CAPS: LetterCase = LetterCase(5);
}

pub struct VerticalSubstitution(pub i8);
impl VerticalSubstitution {
    pub const ON: VerticalSubstitution = VerticalSubstitution(0);
    pub const OFF: VerticalSubstitution = VerticalSubstitution(1);
}

pub struct LinguisticRearrangementType(pub i8);
impl LinguisticRearrangementType {
    pub const ON: LinguisticRearrangementType = LinguisticRearrangementType(0);
    pub const OFF: LinguisticRearrangementType = LinguisticRearrangementType(1);
}

pub struct NumberSpacing(pub i8);
impl NumberSpacing {
    pub const MONOSPACED: NumberSpacing = NumberSpacing(0);
    pub const PROPORTIONAL: NumberSpacing = NumberSpacing(1);
    pub const THIRD_WIDTH: NumberSpacing = NumberSpacing(2);
    pub const QUARTER_WIDTH: NumberSpacing = NumberSpacing(3);
}

pub struct SmartSwashType(pub i8);
impl SmartSwashType {
    pub const WORD_INITIAL_SWASHES_ON: SmartSwashType = SmartSwashType(0);
    pub const WORD_INITIAL_SWASHES_OFF: SmartSwashType = SmartSwashType(1);
    pub const WORD_FINAL_SWASHES_ON: SmartSwashType = SmartSwashType(2);
    pub const WORD_FINAL_SWASHES_OFF: SmartSwashType = SmartSwashType(3);

    pub const LINE_INITIAL_SWASHES_ON: SmartSwashType = SmartSwashType(4);
    pub const LINE_INITIAL_SWASHES_OFF: SmartSwashType = SmartSwashType(5);
    pub const LINE_FINAL_SWASHES_ON: SmartSwashType = SmartSwashType(6);
    pub const LINE_FINAL_SWASHES_OFF: SmartSwashType = SmartSwashType(7);

    pub const NON_FINAL_SWASHES_ON: SmartSwashType = SmartSwashType(8);
    pub const NON_FINAL_SWASHES_OFF: SmartSwashType = SmartSwashType(9);
}

pub struct DiacriticsType(pub i8);
impl DiacriticsType {
    pub const SHOW_DIACRITICS: DiacriticsType = DiacriticsType(0);
    pub const HIDE_DIACRITICS: DiacriticsType = DiacriticsType(1);
    pub const DECOMPOSE_DIACRITICS: DiacriticsType = DiacriticsType(2);
}

pub struct VerticalPosition(pub i8);
impl VerticalPosition {
    pub const NORMAL_POSITION: VerticalPosition = VerticalPosition(0);
    pub const SUPERIORS: VerticalPosition = VerticalPosition(1);
    pub const INFERIORS: VerticalPosition = VerticalPosition(2);
    pub const ORDINALS: VerticalPosition = VerticalPosition(3);
    pub const SCIENTIFIC_INFERIORS: VerticalPosition = VerticalPosition(4);
}

pub struct FractionType(pub i8);
impl FractionType {
    pub const NO_FRACTIONS: FractionType = FractionType(0);
    pub const VERTICAL_FRACTIONS: FractionType = FractionType(1);
    pub const DIAGONAL_FRACTIONS: FractionType = FractionType(2);
}

pub struct OverlappingCharactersType(pub i8);
impl OverlappingCharactersType {
    pub const PREVENT_OVERLAP_ON: OverlappingCharactersType = OverlappingCharactersType(0);
    pub const PREVENT_OVERLAP_OFF: OverlappingCharactersType = OverlappingCharactersType(1);
}

pub struct TypographicExtrasType(pub i8);
impl TypographicExtrasType {
    pub const HYPHENS_TO_EM_DASH_ON: TypographicExtrasType = TypographicExtrasType(0);
    pub const HYPHENS_TO_EM_DASH_OFF: TypographicExtrasType = TypographicExtrasType(1);
    pub const HYPHEN_TO_EN_DASH_ON: TypographicExtrasType = TypographicExtrasType(2);
    pub const HYPHEN_TO_EN_DASH_OFF: TypographicExtrasType = TypographicExtrasType(3);
    pub const SLASHED_ZERO_ON: TypographicExtrasType = TypographicExtrasType(4);
    pub const SLASHED_ZERO_OFF: TypographicExtrasType = TypographicExtrasType(5);
    pub const FORM_INTERROBANG_ON: TypographicExtrasType = TypographicExtrasType(6);
    pub const FORM_INTERROBANG_OFF: TypographicExtrasType = TypographicExtrasType(7);
    pub const SMART_QUOTES_ON: TypographicExtrasType = TypographicExtrasType(8);
    pub const SMART_QUOTES_OFF: TypographicExtrasType = TypographicExtrasType(9);
    pub const PERIODS_TO_ELLIPSIS_ON: TypographicExtrasType = TypographicExtrasType(10);
    pub const PERIODS_TO_ELLIPSIS_OFF: TypographicExtrasType = TypographicExtrasType(11);
}

pub struct MathematicalExtrasType(pub i8);
impl MathematicalExtrasType {
    pub const HYPHEN_TO_MINUS_ON: MathematicalExtrasType = MathematicalExtrasType(0);
    pub const HYPHEN_TO_MINUS_OFF: MathematicalExtrasType = MathematicalExtrasType(1);
    pub const ASTERISK_TO_MULTIPLY_ON: MathematicalExtrasType = MathematicalExtrasType(2);
    pub const ASTERISK_TO_MULTIPLY_OFF: MathematicalExtrasType = MathematicalExtrasType(3);
    pub const SLASH_TO_DIVIDE_ON: MathematicalExtrasType = MathematicalExtrasType(4);
    pub const SLASH_TO_DIVIDE_OFF: MathematicalExtrasType = MathematicalExtrasType(5);
    pub const INEQUALITY_LIGATURES_ON: MathematicalExtrasType = MathematicalExtrasType(6);
    pub const INEQUALITY_LIGATURES_OFF: MathematicalExtrasType = MathematicalExtrasType(7);
    pub const EXPONENTS_ON: MathematicalExtrasType = MathematicalExtrasType(8);
    pub const EXPONENTS_OFF: MathematicalExtrasType = MathematicalExtrasType(9);
    pub const MATHEMATICAL_GREEK_ON: MathematicalExtrasType = MathematicalExtrasType(10);
    pub const MATHEMATICAL_GREEK_OFF: MathematicalExtrasType = MathematicalExtrasType(11);
}

pub struct OrnamentSetsType(pub i8);
impl OrnamentSetsType {
    pub const NO_ORNAMENTS: OrnamentSetsType = OrnamentSetsType(0);
    pub const DINGBATS: OrnamentSetsType = OrnamentSetsType(1);
    pub const PI_CHARACTERS: OrnamentSetsType = OrnamentSetsType(2);
    pub const FLEURONS: OrnamentSetsType = OrnamentSetsType(3);
    pub const DECORATIVE_BORDERS: OrnamentSetsType = OrnamentSetsType(4);
    pub const INTERNATIONAL_SYMBOLS: OrnamentSetsType = OrnamentSetsType(5);
    pub const MATH_SYMBOLS: OrnamentSetsType = OrnamentSetsType(6);
}

pub struct CharacterAlternativesType(pub i8);
impl CharacterAlternativesType {
    pub const NO_ALTERNATES: CharacterAlternativesType = CharacterAlternativesType(0);
}

pub struct DesignComplexityType(pub i8);
impl DesignComplexityType {
    pub const DESIGN_LEVEL_1: DesignComplexityType = DesignComplexityType(0);
    pub const DESIGN_LEVEL_2: DesignComplexityType = DesignComplexityType(1);
    pub const DESIGN_LEVEL_3: DesignComplexityType = DesignComplexityType(2);
    pub const DESIGN_LEVEL_4: DesignComplexityType = DesignComplexityType(3);
    pub const DESIGN_LEVEL_5: DesignComplexityType = DesignComplexityType(4);
}

pub struct StyleOptionsType(pub i8);
impl StyleOptionsType {
    pub const NO_STYLE_OPTIONS: StyleOptionsType = StyleOptionsType(0);
    pub const DISPLAY_TEXT: StyleOptionsType = StyleOptionsType(1);
    pub const ENGRAVED_TEXT: StyleOptionsType = StyleOptionsType(2);
    pub const ILLUMINATED_CAPS: StyleOptionsType = StyleOptionsType(3);
    pub const TITLING_CAPS: StyleOptionsType = StyleOptionsType(4);
    pub const TALL_CAPS: StyleOptionsType = StyleOptionsType(5);
}

pub struct CharacterShapeType(pub i8);
impl CharacterShapeType {
    pub const TRADITIONAL_CHARACTERS: CharacterShapeType = CharacterShapeType(0);
    pub const SIMPLIFIED_CHARACTERS: CharacterShapeType = CharacterShapeType(1);
    pub const JIS1978_CHARACTERS: CharacterShapeType = CharacterShapeType(2);
    pub const JIS1983_CHARACTERS: CharacterShapeType = CharacterShapeType(3);
    pub const JIS1990_CHARACTERS: CharacterShapeType = CharacterShapeType(4);
    pub const TRADITIONAL_ALT_ONE: CharacterShapeType = CharacterShapeType(5);
    pub const TRADITIONAL_ALT_TWO: CharacterShapeType = CharacterShapeType(6);
    pub const TRADITIONAL_ALT_THREE: CharacterShapeType = CharacterShapeType(7);
    pub const TRADITIONAL_ALT_FOUR: CharacterShapeType = CharacterShapeType(8);
    pub const TRADITIONAL_ALT_FIVE: CharacterShapeType = CharacterShapeType(9);
    pub const EXPERT_CHARACTERS: CharacterShapeType = CharacterShapeType(10);
    pub const JIS2004_CHARACTERS: CharacterShapeType = CharacterShapeType(11);
    pub const HOJO_CHARACTERS: CharacterShapeType = CharacterShapeType(12);
    pub const NLCCHARACTERS: CharacterShapeType = CharacterShapeType(13);
    pub const TRADITIONAL_NAMES_CHARACTERS: CharacterShapeType = CharacterShapeType(14);
}

pub struct NumberCaseType(pub i8);
impl NumberCaseType {
    pub const LOWER_CASE_NUMBERS: NumberCaseType = NumberCaseType(0);
    pub const UPPER_CASE_NUMBERS: NumberCaseType = NumberCaseType(1);
}

pub struct TextSpacingType(pub i8);
impl TextSpacingType {
    pub const PROPORTIONAL_TEXT: TextSpacingType = TextSpacingType(0);
    pub const MONOSPACED_TEXT: TextSpacingType = TextSpacingType(1);
    pub const HALF_WIDTH_TEXT: TextSpacingType = TextSpacingType(2);
    pub const THIRD_WIDTH_TEXT: TextSpacingType = TextSpacingType(3);
    pub const QUARTER_WIDTH_TEXT: TextSpacingType = TextSpacingType(4);
    pub const ALT_PROPORTIONAL_TEXT: TextSpacingType = TextSpacingType(5);
    pub const ALT_HALF_WIDTH_TEXT: TextSpacingType = TextSpacingType(6);
}

pub struct TransliterationType(pub i8);
impl TransliterationType {
    pub const NO_TRANSLITERATION: TransliterationType = TransliterationType(0);
    pub const HANJA_TO_HANGUL: TransliterationType = TransliterationType(1);
    pub const HIRAGANA_TO_KATAKANA: TransliterationType = TransliterationType(2);
    pub const KATAKANA_TO_HIRAGANA: TransliterationType = TransliterationType(3);
    pub const KANA_TO_ROMANIZATION: TransliterationType = TransliterationType(4);
    pub const ROMANIZATION_TO_HIRAGANA: TransliterationType = TransliterationType(5);
    pub const ROMANIZATION_TO_KATAKANA: TransliterationType = TransliterationType(6);
    pub const HANJA_TO_HANGUL_ALT_ONE: TransliterationType = TransliterationType(7);
    pub const HANJA_TO_HANGUL_ALT_TWO: TransliterationType = TransliterationType(8);
    pub const HANJA_TO_HANGUL_ALT_THREE: TransliterationType = TransliterationType(9);
}

pub struct AnnotationType(pub i8);
impl AnnotationType {
    pub const NO: AnnotationType = AnnotationType(0);
    pub const BOX: AnnotationType = AnnotationType(1);
    pub const ROUNDED_BOX: AnnotationType = AnnotationType(2);
    pub const CIRCLE: AnnotationType = AnnotationType(3);
    pub const INVERTED_CIRCLE: AnnotationType = AnnotationType(4);
    pub const PARENTHESIS: AnnotationType = AnnotationType(5);
    pub const PERIOD: AnnotationType = AnnotationType(6);
    pub const ROMAN_NUMERAL: AnnotationType = AnnotationType(7);
    pub const DIAMOND: AnnotationType = AnnotationType(8);
    pub const INVERTED_BOX: AnnotationType = AnnotationType(9);
    pub const INVERTED_ROUNDED_BOX: AnnotationType = AnnotationType(10);
}

pub struct KanaSpacingType(pub i8);
impl KanaSpacingType {
    pub const FULL_WIDTH_KANA: KanaSpacingType = KanaSpacingType(0);
    pub const PROPORTIONAL_KANA: KanaSpacingType = KanaSpacingType(1);
}

pub struct IdeographicSpacingType(pub i8);
impl IdeographicSpacingType {
    pub const FULL_WIDTH_IDEOGRAPHS: IdeographicSpacingType = IdeographicSpacingType(0);
    pub const PROPORTIONAL_IDEOGRAPHS: IdeographicSpacingType = IdeographicSpacingType(1);
    pub const HALF_WIDTH_IDEOGRAPHS: IdeographicSpacingType = IdeographicSpacingType(2);
}

pub struct UnicodeDecompositionType(pub i8);
impl UnicodeDecompositionType {
    pub const CANONICAL_COMPOSITION_ON: UnicodeDecompositionType = UnicodeDecompositionType(0);
    pub const CANONICAL_COMPOSITION_OFF: UnicodeDecompositionType = UnicodeDecompositionType(1);
    pub const COMPATABILTY_COMPOSITION_ON: UnicodeDecompositionType = UnicodeDecompositionType(2);
    pub const COMPATABILTY_COMPOSITION_OFF: UnicodeDecompositionType = UnicodeDecompositionType(3);
    pub const TRANSCODING_COMPOSITION_ON: UnicodeDecompositionType = UnicodeDecompositionType(4);
    pub const TRANSCODING_COMPOSITION_OFF: UnicodeDecompositionType = UnicodeDecompositionType(5);
}

pub struct RubyKanaType(pub i8);
impl RubyKanaType {
    pub const NO_RUBY_KANA: RubyKanaType = RubyKanaType(0);
    pub const RUBY_KANA: RubyKanaType = RubyKanaType(1);
    pub const RUBY_KANA_ON: RubyKanaType = RubyKanaType(2);
    pub const RUBY_KANA_OFF: RubyKanaType = RubyKanaType(3);
}

pub struct CJKSymbolAlternativesType(pub i8);
impl CJKSymbolAlternativesType {
    pub const NO_ALTERNATIVES: CJKSymbolAlternativesType = CJKSymbolAlternativesType(0);
    pub const ALT_ONE: CJKSymbolAlternativesType = CJKSymbolAlternativesType(1);
    pub const ALT_TWO: CJKSymbolAlternativesType = CJKSymbolAlternativesType(2);
    pub const ALT_THREE: CJKSymbolAlternativesType = CJKSymbolAlternativesType(3);
    pub const ALT_FOUR: CJKSymbolAlternativesType = CJKSymbolAlternativesType(4);
    pub const ALT_FIVE: CJKSymbolAlternativesType = CJKSymbolAlternativesType(5);
}

pub struct IdeographicAlternativesType(pub i8);
impl IdeographicAlternativesType {
    pub const NO_ALTERNATIVES: IdeographicAlternativesType = IdeographicAlternativesType(0);
    pub const ALT_ONE: IdeographicAlternativesType = IdeographicAlternativesType(1);
    pub const ALT_TWO: IdeographicAlternativesType = IdeographicAlternativesType(2);
    pub const ALT_THREE: IdeographicAlternativesType = IdeographicAlternativesType(3);
    pub const ALT_FOUR: IdeographicAlternativesType = IdeographicAlternativesType(4);
    pub const ALT_FIVE: IdeographicAlternativesType = IdeographicAlternativesType(5);
}

pub struct CJKVerticalRomanPlacementType(pub i8);
impl CJKVerticalRomanPlacementType {
    pub const CENTERED: CJKVerticalRomanPlacementType = CJKVerticalRomanPlacementType(0);
    pub const H_BASELINE: CJKVerticalRomanPlacementType = CJKVerticalRomanPlacementType(1);
}

pub struct ItalicCJKRomanType(pub i8);
impl ItalicCJKRomanType {
    pub const NO_ITALIC: ItalicCJKRomanType = ItalicCJKRomanType(0);
    pub const ITALIC: ItalicCJKRomanType = ItalicCJKRomanType(1);
    pub const ON: ItalicCJKRomanType = ItalicCJKRomanType(2);
    pub const OFF: ItalicCJKRomanType = ItalicCJKRomanType(3);
}

pub struct CaseSensitiveLayoutType(pub i8);
impl CaseSensitiveLayoutType {
    pub const ON: CaseSensitiveLayoutType = CaseSensitiveLayoutType(0);
    pub const OFF: CaseSensitiveLayoutType = CaseSensitiveLayoutType(1);
    pub const SPACING_ON: CaseSensitiveLayoutType = CaseSensitiveLayoutType(2);
    pub const SPACING_OFF: CaseSensitiveLayoutType = CaseSensitiveLayoutType(3);
}

pub struct AlternateKanaType(pub i8);
impl AlternateKanaType {
    pub const HORIZ_KANA_ON: AlternateKanaType = AlternateKanaType(0);
    pub const HORIZ_KANA_OFF: AlternateKanaType = AlternateKanaType(1);
    pub const VERT_KANA_ON: AlternateKanaType = AlternateKanaType(2);
    pub const VERT_KANA_OFF: AlternateKanaType = AlternateKanaType(3);
}

pub struct StylisticAlternativesType(pub i8);
impl StylisticAlternativesType {
    pub const NO_ALTERNATIVES: StylisticAlternativesType = StylisticAlternativesType(0);
    pub const ALT_ONE_ON: StylisticAlternativesType = StylisticAlternativesType(2);
    pub const ALT_ONE_OFF: StylisticAlternativesType = StylisticAlternativesType(3);
    pub const ALT_TWO_ON: StylisticAlternativesType = StylisticAlternativesType(4);
    pub const ALT_TWO_OFF: StylisticAlternativesType = StylisticAlternativesType(5);
    pub const ALT_THREE_ON: StylisticAlternativesType = StylisticAlternativesType(6);
    pub const ALT_THREE_OFF: StylisticAlternativesType = StylisticAlternativesType(7);
    pub const ALT_FOUR_ON: StylisticAlternativesType = StylisticAlternativesType(8);
    pub const ALT_FOUR_OFF: StylisticAlternativesType = StylisticAlternativesType(9);
    pub const ALT_FIVE_ON: StylisticAlternativesType = StylisticAlternativesType(10);
    pub const ALT_FIVE_OFF: StylisticAlternativesType = StylisticAlternativesType(11);
    pub const ALT_SIX_ON: StylisticAlternativesType = StylisticAlternativesType(12);
    pub const ALT_SIX_OFF: StylisticAlternativesType = StylisticAlternativesType(13);
    pub const ALT_SEVEN_ON: StylisticAlternativesType = StylisticAlternativesType(14);
    pub const ALT_SEVEN_OFF: StylisticAlternativesType = StylisticAlternativesType(15);
    pub const ALT_EIGHT_ON: StylisticAlternativesType = StylisticAlternativesType(16);
    pub const ALT_EIGHT_OFF: StylisticAlternativesType = StylisticAlternativesType(17);
    pub const ALT_NINE_ON: StylisticAlternativesType = StylisticAlternativesType(18);
    pub const ALT_NINE_OFF: StylisticAlternativesType = StylisticAlternativesType(19);
    pub const ALT_TEN_ON: StylisticAlternativesType = StylisticAlternativesType(20);
    pub const ALT_TEN_OFF: StylisticAlternativesType = StylisticAlternativesType(21);
    pub const ALT_ELEVEN_ON: StylisticAlternativesType = StylisticAlternativesType(22);
    pub const ALT_ELEVEN_OFF: StylisticAlternativesType = StylisticAlternativesType(23);
    pub const ALT_TWELVE_ON: StylisticAlternativesType = StylisticAlternativesType(24);
    pub const ALT_TWELVE_OFF: StylisticAlternativesType = StylisticAlternativesType(25);
    pub const ALT_THIRTEEN_ON: StylisticAlternativesType = StylisticAlternativesType(26);
    pub const ALT_THIRTEEN_OFF: StylisticAlternativesType = StylisticAlternativesType(27);
    pub const ALT_FOURTEEN_ON: StylisticAlternativesType = StylisticAlternativesType(28);
    pub const ALT_FOURTEEN_OFF: StylisticAlternativesType = StylisticAlternativesType(29);
    pub const ALT_FIFTEEN_ON: StylisticAlternativesType = StylisticAlternativesType(30);
    pub const ALT_FIFTEEN_OFF: StylisticAlternativesType = StylisticAlternativesType(31);
    pub const ALT_SIXTEEN_ON: StylisticAlternativesType = StylisticAlternativesType(32);
    pub const ALT_SIXTEEN_OFF: StylisticAlternativesType = StylisticAlternativesType(33);
    pub const ALT_SEVENTEEN_ON: StylisticAlternativesType = StylisticAlternativesType(34);
    pub const ALT_SEVENTEEN_OFF: StylisticAlternativesType = StylisticAlternativesType(35);
    pub const ALT_EIGHTEEN_ON: StylisticAlternativesType = StylisticAlternativesType(36);
    pub const ALT_EIGHTEEN_OFF: StylisticAlternativesType = StylisticAlternativesType(37);
    pub const ALT_NINETEEN_ON: StylisticAlternativesType = StylisticAlternativesType(38);
    pub const ALT_NINETEEN_OFF: StylisticAlternativesType = StylisticAlternativesType(39);
    pub const ALT_TWENTY_ON: StylisticAlternativesType = StylisticAlternativesType(40);
    pub const ALT_TWENTY_OFF: StylisticAlternativesType = StylisticAlternativesType(41);
}

pub struct ContextualAlternatesType(pub i8);
impl ContextualAlternatesType {
    pub const CONTEXTUAL_ALTERNATES_ON: ContextualAlternatesType = ContextualAlternatesType(0);
    pub const CONTEXTUAL_ALTERNATES_OFF: ContextualAlternatesType = ContextualAlternatesType(1);
    pub const SWASH_ALTERNATES_ON: ContextualAlternatesType = ContextualAlternatesType(2);
    pub const SWASH_ALTERNATES_OFF: ContextualAlternatesType = ContextualAlternatesType(3);
    pub const CONTEXTUAL_SWASH_ALTERNATES_ON: ContextualAlternatesType = ContextualAlternatesType(4);
    pub const CONTEXTUAL_SWASH_ALTERNATES_OFF: ContextualAlternatesType = ContextualAlternatesType(5);
}

pub struct LowerCaseType(pub i8);
impl LowerCaseType {
    pub const DEFAULT: LowerCaseType = LowerCaseType(0);
    pub const SMALL_CAPS: LowerCaseType = LowerCaseType(1);
    pub const PETITE_CAPS: LowerCaseType = LowerCaseType(2);
}

pub struct UpperCaseType(pub i8);
impl UpperCaseType {
    pub const DEFAULT: UpperCaseType = UpperCaseType(0);
    pub const SMALL_CAPS: UpperCaseType = UpperCaseType(1);
    pub const PETITE_CAPS: UpperCaseType = UpperCaseType(2);
}

pub struct CJKRomanSpacingType(pub i8);
impl CJKRomanSpacingType {
    pub const HALF_WIDTH: CJKRomanSpacingType = CJKRomanSpacingType(0);
    pub const PROPORTIONAL_WIDTH: CJKRomanSpacingType = CJKRomanSpacingType(1);
    pub const DEFAULT: CJKRomanSpacingType = CJKRomanSpacingType(2);
    pub const FULL_WIDTH: CJKRomanSpacingType = CJKRomanSpacingType(3);
}





