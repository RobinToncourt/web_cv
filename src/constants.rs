// TODO
// -Rajouter context.
// -Traduire.
pub static JSON: &str = r#"{
    "fr": {
        "exp":
        {
            "AKKODIS": "De février 2023 à ce jour : Akkodis",

            "KNDS_BOURGES_ID": "knds_bourges_top_ready",
            "KNDS_BOURGES_TITLE": "Client : KNDS (ex-Nexter) à Bourges",
            "KNDS_BOURGES_DATES": "Janvier 2025 à ce jour",
            "KNDS_BOURGES_MISSION": "Projet Top&Ready
KNDS demande à ce que le logiciel historique TOP&READY, deux logiciels installé sur le CAESAR, soientt modernisés.
Pour ce faire les différentes fonctionnalitées sur transformé en services et .
Cela permettra de faire plus facilement évoluer les logiciels ainsi qu'une meilleur maintenabilité.
Il est aussi envisagé de se séparer de Microsoft MFC pour permettre une migration vers linux.
J'ai du implémenter un service de communication entre le logiciel principal et la central inertielle.",
            "KNDS_BOURGES_TECHNOS": "Technos: C++, Git, Jenkins",

            "THALES_ORLEANS_ID": "thales_orleans_trusttool",
            "THALES_ORLEANS_TITLE": "Client : Thales à Fleury-les-Aubrais",
            "THALES_ORLEANS_DATES": "Septembre 2024 à décembre 2024",
            "THALES_ORLEANS_MISSION": "Projet : TrustTool
Mise en place de séquences de tests pour le logiciel PAAMS sur l’outil Trust.
Le PAAMS (Principal Anti-Air Missile System) est le logiciel de protection antiaérienne navale employant le missile Aster.
Le logiciel à subit une mise à niveau et les des tests doivent être réalisé, ces tests se fond par l’intermédiaire de l’outil Trust.
Trust est un outil d’automatisation des tests via des séquences de déclenchement d’envoi de messages à intervalles défini par l’opérateur.
Le développement des tests se font via l’IHM sur navigateur internet.",
            "THALES_ORLEANS_TECHNOS": "Technos: Java, Python, Ada, Git, Jira, Agilité",

            "THALES_ETRELLES_ID": "thales_etrelles_b1nt_awg",
            "THALES_ETRELLES_TITLE": "Client : Thales à Etrelles",
            "THALES_ETRELLES_DATES": "",
            "THALES_ETRELLES_MISSION": "Projet : B1NT-AWG
Développement d’une DLL et de séquences puis intégration sur le banc de test.
Le banc B1NT-AWG doit vérifier le bon fonctionnement de la carte AWG, pour ce faire des appareils sont disponibles pour contrôler les entrées et sorties de la carte.
Dans un premier temps l’objectif est de produire une DLL faisant office d’interface entre les séquences TestStand et les différents appareils présents sur le banc.
Ensuite utiliser cette DLL pour automatiser les tests via des séquences sur TestStand.
Pour finir, l’intégration des composants sur le banc pour vérifier le bon fonctionnement et corriger les éventuels problèmes.",
            "THALES_ETRELLES_TECHNOS": "Technos: C, TestStand, Jira, Agilité",

            "THALES_BREST_TBSL_ID": "thales_brest_tbsl_hmi",
            "THALES_BREST_TBSL_TITLE": "Client : Thales à Brest",
            "THALES_BREST_TBSL_DATES": "2 semaines",
            "THALES_BREST_TBSL_MISSION": "Projet : TBSL HMI
Portage d’une librairie propriétaire Thales de Linux vers Windows
Le projet porte sur le portage, de Linux vers Windows, d’un logiciel de test de banc de carte embarquée dans une gamme de sonar bouée (Thales TUS).
Le logiciel, écrit en Java, utilise une librairie native pour communiquer avec le matériel. J’ai dû faire le portage de cette librairie vers Windows.
Lors de cette mission, j’ai pu monter en compétence sur la migration de librairie de Linux vers Windows ainsi que sur la compilation de DLL.",
            "THALES_BREST_TBSL_TECHNOS": "Technos: C, Linux, Windows, Visual Studio C++",

            "THALES_BREST_SEA_ID": "thales_brest_sea_guardian",
            "THALES_BREST_SEA_TITLE": "Client : Thales à Brest",
            "THALES_BREST_SEA_DATES": "",
            "THALES_BREST_SEA_MISSION": "Projet : Sea Guardian
Développeur Logiciel
Le projet Sea Guardian est rattaché au service AEROPORTE de l’entité UWS (Under Water Systems) de Thales DMS.
Ce projet a pour objectif de remplacer l’ATL2 par un drone, le but est que le drone ne soit qu’un relai entre les bouées et le centre de traitement des informations.
Je me suis occupé de la partie communication entre la partie vol (drone) et la partie sol (centre de traitement).
Sur cette mission, j’ai pu approfondir mes compétences en C++ et en agilité.",
            "THALES_BREST_SEA_TECHNOS": "Technos: C++, Git, Jira, Agilité",

            "SII": "Janvier à septembre 2022 : SII",

            "THALES_BREST_MENACE_ID": "thales_brest_menace",
            "THALES_BREST_MENACE_TITLE": "Client : Thales à Brest",
            "THALES_BREST_MENACE_DATES": "",
            "THALES_BREST_MENACE_MISSION": "Projet : Menace
Développeur Logiciel
Menace est une application de référencement des objets détectés. Mon implication sur ce projet a été de faire de la maintenance et des corrections de faits techniques, j’ai aussi dû faire des tests unitaires, des tests fonctionnels ainsi que des tests de non-régression.",
            "THALES_BREST_MENACE_TECHNOS": "Technos : Java, JavaFx, Java Swing, Git, Agilité",

            "THALES_ALTERNANCE": "Octobre 2018 à septembre 2021 : Thales en alternance",

            "THALES_BREST_SAMDIS_ID": "thales_brest_samdis",
            "THALES_BREST_SAMDIS_TITLE": "Client : Thales à Brest",
            "THALES_BREST_SAMDIS_DATES": "",
            "THALES_BREST_SAMDIS_MISSION": "Projet : SAMDIS data analysis
Développeur Logiciel
L’objectif de ce projet est de fournir une aide à l’analyse et au traitement de fichiers, documents, bases de données pour le développement du produit SAMDIS, développé dans le service Guerre Des Mines d’UWS.
L’idée retenue a été un logiciel en Java principalement constitué d’une interface graphique, en JavaFX, extensible par des plugins, ces plugins sont conçus et implémentés par l’utilisateur.
Pour assurer une compatibilité avec les différents types de fichiers et librairies existantes, le logiciel supporte des plugins développés en Java ou en C/C++.
Au cours du développement a été rajouté la possibilité de créer des graphiques pour comparer l’évolution de données dans le temps.
Lors de ce projet j’ai beaucoup monté en compétence sur JavaFX et sur la conception d’IHM.",
            "THALES_BREST_SAMDIS_TECHNOS": "Technos : Java, Javafx, C, JNI, SqLite",

            "THALES_BREST_CPP_TO_JAVA_ID": "thales_brest_cpp_to_java",
            "THALES_BREST_CPP_TO_JAVA_TITLE": "Client : Thales à Brest",
            "THALES_BREST_CPP_TO_JAVA_DATES": "",
            "THALES_BREST_CPP_TO_JAVA_MISSION": "Projet : Étude des performances de Java
Développeur Logiciel
Le but de ce projet était de tester les performances de Java, dans un premier temps, puis de les comparer avec le C++. On m’a donc demandé de traduire, du C++ vers le Java, un programme pour ensuite faire différents tests de charge sur les deux versions et de les comparer. Les comparaisons ont été faites sur le temps d’exécution des méthodes importantes ainsi que sur la mémoire utilisée par la JVM. J’ai aussi comparé les différents ramasse miettes, leurs efficacités et leurs vitesses d’exécutions.",
            "THALES_BREST_CPP_TO_JAVA_TECHNOS": "Technos : Java, C, C++, JNI",

            "KERPAPE": "[DATES] : Kerpape en stage de BTS",

            "KERPAPE_LORIENT_I2_ID": "kerpape_lorient_i2",
            "KERPAPE_LORIENT_I2_TITLE": "Client : Kerpape à Lorient",
            "KERPAPE_LORIENT_I2_DATES": "",
            "KERPAPE_LORIENT_I2_MISSION": "Projet : Amélioration et évolution de l’intranet de l’hôpital.
Développeur Logiciel
Lors de mes stages de BTS dans le service informatique, j’ai dû créer une interface WEB, HTML, CSS, JavaScript, pour faciliter et assister les collaborateurs dans leurs tâches.",
            "KERPAPE_LORIENT_I2_TECHNOS": "Technos : PHP, JavaScript, Oracle DB, MySql",

            "KERPAPE_LORIENT_SUMMER_ID": "kerpape_lorient_summer",
            "KERPAPE_LORIENT_SUMMER_TITLE": "Client : Kerpape à Lorient",
            "KERPAPE_LORIENT_SUMMER_DATES": "",
            "KERPAPE_LORIENT_SUMMER_MISSION": "",
            "KERPAPE_LORIENT_SUMMER_TECHNOS": "Technos : PHP, JavaScript, Oracle DB, MySql",

            "KERPAPE_LORIENT_I1_ID": "kerpape_lorient_i1",
            "KERPAPE_LORIENT_I1_TITLE": "Client : Kerpape à Lorient",
            "KERPAPE_LORIENT_I1_DATES": "",
            "KERPAPE_LORIENT_I1_MISSION": "",
            "KERPAPE_LORIENT_I1_TECHNOS": "Technos : PHP, JavaScript, Oracle DB, MySql"
        },
        "skills": {
            "PROG_LANG": [
                "🡢Rust ★★★★★
Rust est mon langage de programmation préféré.",
                "🡢C/C++ ★★★★☆",
                "🡢Java ★★★☆☆
Le langage avec lequel j'ai vraiment commencé la programmation au lycée.",
                "🡢Python ★★★☆☆",
                "🡢Ada ★★☆☆☆"
            ],
            "TOOLS": [
                "✎SqLite",
                "✎Oracle",
                "✎MySql"
            ],
            "ENV": [
                "∞Eclipse",
                "∞QtCreator",
                "∞Visual Studio 2017"
            ],
            "LANG": [
                "✓Français, natif",
                "✓Anglais, courant"
            ]
        },
        "edu": {
            "ADA": " • 2025 : Formation Ada par Robert Cholay",
            "EPSI": " • 2018-2021 : Master expert en informatique et systèmes d'informations - EPSI - Brest",
            "BTS": " • 2016-2018 : BTS Service Informatique aux Organisation option Solutions Logicielles et applications métiers"
        }
    }
}
"#;
