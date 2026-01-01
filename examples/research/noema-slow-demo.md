```yaml
# ontotype: onto-form/v2
# Демонстрация перехода NoemaFast → NoemaSlow при этической неопределённости

meta:
  name: "NoemaSlow Transition Demo"
  version: "1.1.0"
  cla_jurisdiction: "ZA"
  aenga_binding: true

ethics:
  aenga_core:
    dignity_preservation: true
    autonomy_respect: true
    non_instrumentalization: true

ontology:
  phases:
    - name: "Fast Narrative Generation"
      density_score: 0.71
      ethical_constraints:
        - "Avoid stereotyping based on zodiac or culture"
        - "Narratives must include exit option"
      phase_context: "gaming-narrative-fast"
      social_proximity: 0.6
      mbti_alignment: "ENFP"

    - name: "Slow Ethical Review"
      density_score: 0.92
      ethical_constraints:
        - "Community-based validation required for high-impact narratives"
        - "Historical bias audit mandatory"
        - "User must confirm before proceeding"
      phase_context: "noema-slow-ethical-review"
      social_proximity: 0.9
      mbti_alignment: "ISTJ"

runtime:
  sgRL_protection: enabled
  vma_trigger: "ethical-ambiguity-threshold > 0.5"
```