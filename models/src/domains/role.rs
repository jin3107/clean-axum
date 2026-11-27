use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(32))")]
pub enum Role {
    #[sea_orm(string_value = "admin")]
    #[serde(rename = "admin")]
    Admin,
{%- assign names = roles | split: "," | uniq %}
{%- for name in names %}
{%- if name != "admin" %}
    #[sea_orm(string_value = "{{ name }}")]
    #[serde(rename = "{{ name }}")]
    {{ name | pascal_case }},
{%- endif %}
{%- endfor %}
}
