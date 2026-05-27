<item ident="ITEM001" title="単一選択問題の例">
  <presentation>
    <material>
      <mattext texttype="text/plain">
        日本の首都はどこですか。
      </mattext>
    </material>

    <response_lid ident="RESPONSE" rcardinality="Single">
      <render_choice shuffle="No">
        <response_label ident="A">
          <material>
            <mattext texttype="text/plain">大阪</mattext>
          </material>
        </response_label>
    
        <response_label ident="B">
          <material>
            <mattext texttype="text/plain">東京</mattext>
          </material>
        </response_label>
    
        <response_label ident="C">
          <material>
            <mattext texttype="text/plain">京都</mattext>
          </material>
        </response_label>
    
        <response_label ident="D">
          <material>
            <mattext texttype="text/plain">福岡</mattext>
          </material>
        </response_label>
      </render_choice>
    </response_lid>
  </presentation>

  <resprocessing>
    <outcomes>
      <decvar vartype="Decimal" minvalue="0" maxvalue="1" varname="SCORE"/>
    </outcomes>

    <respcondition continue="No">
      <conditionvar>
        <varequal respident="RESPONSE">B</varequal>
      </conditionvar>
      <setvar varname="SCORE" action="Set">1</setvar>
    </respcondition>
  </resprocessing>
</item>