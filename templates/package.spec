Name:           {{ name }}
Version:        {{ version }}
Release:        0%{?dist}
Summary:        {{ summary }}
License:        {{ license }}

URL:            {{ url }}
Source0:        {{ source }}

BuildArch:      noarch

BuildRequires:  maven-local
BuildRequires:  mvn-genbr

%description
{{ description }}

%prep
%autosetup -p1

{% for module in modules %}
%mvn_package {{ module }}
{% endfor -%}

%generate_buildrequires
mvn-genbr

{% for module in modules %}
mvn-genbr {{ module }}
{% endfor -%}

%build
%mvn_build

%install
{%- if with_tests %}
%mvn_install
{%- else %}
%mvn_install -f
{%- endif %}

%files -f .mfiles

%changelog
