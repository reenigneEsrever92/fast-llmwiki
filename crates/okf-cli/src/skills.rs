//! Agent skills embedded in the `okf` binary.
//!
//! Skills ship in this repository under `.agents/skills/<name>/SKILL.md`.
//! Rather than reading them from disk at runtime, they are baked into the
//! binary with [`include_str!`] so `okf install` works from a standalone
//! binary without the repository checkout present.

/// A single bundled skill: its directory name and the contents of its
/// `SKILL.md`.
pub struct Skill {
    pub name: &'static str,
    pub content: &'static str,
}

/// Embed every skill shipped in this repository into the binary.
///
/// Each entry pairs a skill's directory name with the path (relative to the
/// workspace root) of its `SKILL.md`. `include_str!` resolves the path at
/// compile time and bakes the file's contents into the `SKILLS` constant, so
/// the install command never touches the filesystem for the source.
macro_rules! include_skills {
    ($($name:literal => $path:expr),* $(,)?) => {
        pub static SKILLS: &[Skill] = &[
            $(Skill {
                name: $name,
                content: include_str!($path),
            }),*
        ];
    };
}

include_skills! {
    "okf-dev" => concat!(env!("CARGO_MANIFEST_DIR"), "/../../.agents/skills/okf-dev/SKILL.md"),
    "okf-plan" => concat!(env!("CARGO_MANIFEST_DIR"), "/../../.agents/skills/okf-plan/SKILL.md"),
    "okf-spec" => concat!(env!("CARGO_MANIFEST_DIR"), "/../../.agents/skills/okf-spec/SKILL.md"),
}

#[cfg(test)]
mod tests {
    use super::SKILLS;

    #[test]
    fn embeds_every_bundled_skill() {
        let names: Vec<&str> = SKILLS.iter().map(|s| s.name).collect();
        assert!(names.contains(&"okf-dev"));
        assert!(names.contains(&"okf-plan"));
        assert!(names.contains(&"okf-spec"));
    }

    #[test]
    fn every_skill_has_a_front_matter_name() {
        for skill in SKILLS {
            assert!(
                skill.content.trim_start().starts_with("---"),
                "skill {} SKILL.md must start with front matter",
                skill.name
            );
            assert!(
                skill.content.contains("name:"),
                "skill {} SKILL.md must declare a name in front matter",
                skill.name
            );
        }
    }

    #[test]
    fn skills_are_unique() {
        let mut names: Vec<&str> = SKILLS.iter().map(|s| s.name).collect();
        names.sort_unstable();
        let deduped_len = {
            let mut v = names.clone();
            v.dedup();
            v.len()
        };
        assert_eq!(names.len(), deduped_len, "skill names must be unique");
    }
}
