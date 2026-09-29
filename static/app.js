/* ============================================================
   SOULSCONFLICT — INTERNATIONALIZATION (i18n) DICTIONARY
   Languages: EN (Primary), PT, IT, FR, JA, ZH
   ============================================================ */

const TRANSLATIONS = {
    en: {
        app_title: "SoulsConflict",
        app_subtitle: "Mod Conflict Checker & Merger — Dark Souls / FromSoftware",
        open_mods_folder: "Open Mods Folder",
        open_mods_folder_title: "Open root folder where all mods are stored",
        tab_merge: "Diagnostic & Merge",
        tab_launcher: "Launcher & DSR Injection",
        status_vanilla: "Vanilla",
        status_modded: "Active",
        load_order_title: "Load Order",
        load_order_desc: "Add as many mod folders as you want. The order defines priority: the mod at the top has highest priority.",
        btn_add_mod: "Add Mod",
        btn_refresh: "Refresh",
        btn_refresh_title: "Refresh file counts",
        btn_import_archive: "Import Mod (.zip / .rar)",
        alert_err_not_archive: "Please select or drop a valid mod archive (.zip, .rar, or .7z).",
        uploading_archive_progress: "Extracting and importing mod '{name}' natively...",
        output_folder_label: "Output Folder:",
        btn_open_folder: "Open Folder",
        btn_run_diagnostic: "Run Diagnostic",
        loading_title: "Analyzing Mods...",
        loading_desc: "Decompressing DCX, matching BND3 tables, FMG text IDs and PARAM parameters.",
        stat_analyzed: "Files Analyzed",
        stat_safe: "Fully Safe",
        stat_mergeable: "Mergeable",
        stat_conflict: "Critical Conflicts",
        ready_merge_title: "Ready to Merge",
        evaluating_msg: "Evaluating compatibility of active mods...",
        all_compatible_msg: "<strong style='color:var(--color-safe);'>100% Compatible.</strong> All {total} mods can be unified without data loss.",
        conflicts_found_msg: "<strong>Attention:</strong> {count} file(s) with conflicting IDs between mods. Choose a resolution strategy below.",
        resolution_strategy_label: "Resolution Strategy:",
        mode_smart_title: "Smart Fusion",
        badge_recommended: "Recommended",
        mode_smart_desc: "Automatically detects extended versions of scripts and map entities.",
        mode_priority_title: "Sequential Priority",
        mode_priority_desc: "In case of collision, the mod higher in the load order wins.",
        mode_manual_title: "Manual Selection",
        mode_manual_desc: "Allows choosing the winning mod individually for each conflict.",
        btn_merge_all: "Merge All Mods",
        tab_filter_all: "All",
        tab_filter_conflict: "Conflicts",
        tab_filter_mergeable: "Mergeable",
        tab_filter_safe: "Safe",
        launcher_title: "Dark Souls: Remastered Integration",
        launcher_desc: "Activate and play your merged mods without permanently altering original game files. Switch between <strong>Modded</strong> and <strong>Vanilla</strong> in 1 click.",
        checking_status: "Checking...",
        exe_found: "Executable Located",
        exe_missing: "Executable Not Found",
        banner_vanilla_title: "VANILLA MODE (ORIGINAL)",
        banner_vanilla_desc: "No game files modified. Factory installation preserved.",
        banner_modded_title: "MODS ACTIVE",
        banner_modded_desc: "{count} merged file(s) activated. Originals safely backed up.",
        banner_active_meta: "Activated at: {time} · Backup in vanilla_backup/",
        banner_ready_meta: "{count} file(s) ready in merged/ folder to apply.",
        active_mods_header: "Active Mods in Fusion (Load Order):",
        active_mods_desc: "The mod at the top (#1) has highest priority in conflicts.",
        game_path_label: "Dark Souls Remastered Installation Directory:",
        game_path_hint: "Must point to the folder containing <code>DarkSoulsRemastered.exe</code>.",
        btn_save: "Save",
        btn_save_title: "Save path",
        btn_open: "Open",
        btn_open_game_folder_title: "Open game folder in Explorer",
        btn_deploy_mods: "Deploy Mods to Game",
        btn_redeploy_mods: "Re-deploy Mods",
        btn_restore_vanilla: "Restore Vanilla",
        btn_launch_game: "Launch DS1",
        btn_launch_seamless: "Launch DS1 Seamless Co-op",
        injected_files_title: "Injected Files",
        injected_files_desc: "Merged files active in game folder with safe backup.",
        no_injected_files: "No modified files active. Game is in Vanilla mode.",
        no_mods_configured: "No mods configured. Go to 'Diagnostic & Merge' tab to add mods.",
        empty_mods_title: "No Mods Found in 'mods/' Folder",
        empty_mods_desc: "SoulsConflict is 100% portable. Simply extract your downloaded mods inside the <strong>mods/</strong> folder. Each mod must be in its own subfolder (e.g. <code>mods/MyMod/chr/...</code>).",
        btn_create_mod_folder: "Create Mod Folder",
        badge_status_backup: "Backed Up",
        badge_status_new: "New",
        badge_ready: "{count} files",
        badge_empty: "Empty",
        badge_auto_mapped: "Auto-Mapped",
        badge_auto_mapped_title: "Files outside standard directories — auto-mapped",
        btn_organize: "Organize",
        btn_organize_title: "Organize loose files in this folder into their canonical directories",
        btn_delete_title: "Delete folder",
        variant_label: "Variant:",
        variant_title: "This mod has alternative variants",
        prompt_new_folder: "Mod folder name (e.g. Cinders, BetterRolling, MyMod):\nLeave blank for automatic name:",
        confirm_delete_folder: "Remove folder 'mods/{id}'?\nAll files inside it will be deleted.",
        confirm_fix_structure: "Automatically organize structure of 'mods/{id}'?\nLoose files will be moved into canonical directories (chr/, param/, event/, etc.).",
        alert_min_mods: "You need at least 2 mod folders to run a conflict diagnostic.",
        alert_merge_success: "Mods merged successfully into 'merged/' folder!",
        alert_restore_confirm: "Remove all active mods and restore original factory files?",
        move_up_title: "Move up priority",
        move_down_title: "Move down priority",
        active_tag: "Active",
        staged_tag: "Staged",
        modal_ok: "OK",
        modal_cancel: "Cancel",
        modal_confirm: "Confirm",
        modal_title_info: "Information",
        modal_title_confirm: "Confirmation",
        modal_title_prompt: "New Mod Folder",
        modal_title_success: "Success",
        modal_title_error: "Error",
        alert_deploy_success: "Mods deployed successfully! {placed} file(s) active in Dark Souls ({backup} original file(s) safely backed up).",
        alert_restore_success: "Vanilla restored successfully! {removed} mod file(s) removed and {restored} original file(s) restored.",
        alert_already_vanilla: "The game is already in original Vanilla state (no active mods).",
        alert_merge_result: "Merge completed successfully ({mode})!\n{copied} unique file(s) copied and {merged} conflicted file(s) unified.",
        alert_fix_structure_success: "Structure organized successfully! {count} file(s) placed into canonical directories.",
        alert_game_path_saved: "Game installation path saved successfully.",
        alert_invalid_path: "Please enter a valid Dark Souls Remastered installation path.",
        alert_err_game_running: "Dark Souls Remastered is currently running! Please close the game before proceeding.",
        alert_err_exe_not_found: "DarkSoulsRemastered.exe was not found at: {path}",
        alert_err_merged_not_found: "The 'merged/' folder does not exist. Run a merge in Diagnostic tab first!",
        alert_err_merged_empty: "The 'merged/' folder is empty! Merge your mods in Diagnostic tab first.",
        alert_err_explorer: "Could not open folder in Explorer.",
        alert_err_organize: "Failed to organize folder.",
        alert_err_create_mod: "Failed to create mod folder.",
        alert_err_scan: "Scan Error: {msg}",
        alert_err_deploy: "Deploy Error: {msg}",
        alert_err_restore: "Restore Error: {msg}",
        alert_err_launch: "Launch Error: {msg}",
        alert_err_generic: "An error occurred: {msg}",
        filter_no_files: "No files found for this filter.",
        launcher_status_active_session: "Current session",
        btn_merging: "Merging...",
        details_discrepancies: "Details & Internal Discrepancies ({count}):",
        present_in: "Present in",
        launch_mode_label: "Launch Method:",
        launch_mode_auto: "Auto-Detect (Steam / Exe)",
        launch_mode_steam: "Steam (steam://run/570940)",
        launch_mode_direct: "Direct Executable (.exe)",
        alert_launch_steam_success: "Launching Dark Souls Remastered via Steam...",
        alert_launch_seamless_success: "Launching Seamless Co-op via launcher...",
        alert_launch_direct_success: "Launching Dark Souls Remastered directly via executable...",
        detected_steam_badge: "Steam Auto-Selected",
        detected_direct_badge: "Non-Steam / Alternative Auto-Selected",
        mod_tag_active: "Active",
        mod_tag_overwritten: "Overwritten",
        mod_tag_combined: "Combined",
        priority_winner_label: "Priority Winner",
        overwritten_disabled_label: "Overwritten / Disabled",
        mergeable_base_label: "Base File (Priority #1)",
        mergeable_combined_label: "Combined in Merge",
        mergeable_banner_hint: "Smart Fusion: Changes from all active mods are combined cleanly without collision.",
        badge_resolved_toggle: "Resolved via Toggle",
        badge_resolved_toggle_title: "This conflict is resolved because a secondary mod was toggled OFF for this file",
        conflict_choice_label: "Active file version:",
        conflict_active_hint: "Check/uncheck a mod to choose which version is included in the merge.",
        custom_toggles_banner: "Custom toggles active ({count} override/exclusions).",
        btn_restore_all_toggles: "Restore Default Priorities",
        val_active_badge: "WINNER",
        val_inactive_badge: "OVERWRITTEN",
        file_toggle_title_on: "Click to toggle this file OFF for this mod",
        file_toggle_title_off: "Click to toggle this file ON for this mod",
        toggle_badge_off: "DISABLED",
        val_active_title: "This value wins and will be used in the merge",
        val_inactive_title: "This value is overwritten and will be discarded",
        overwritten_mods_label: "Overwritten Mods",
        dsrr_preset_title: "Visual Layer Preset (Dark Souls Re-Remastered)",
        dsrr_preset_tag_detected: "DSRR Detected",
        dsrr_preset_desc: "Choose an automatic preset to combine DSRR with overhaul mods without crashes or loading screen freezes.",
        dsrr_preset_btn_enable: "Preset Standard (Logic)",
        dsrr_preset_btn_enable_strict: "Preset Strict (Zero Conflicts / Pure Assets)",
        dsrr_preset_active_title: "Visual Layer Active ({count} logic conflicts bypassed)",
        dsrr_preset_active_title_strict: "Strict Visual Layer Active ({count} files bypassed — Zero Conflicts)",
        dsrr_preset_active_tag: "Visual Layer (Standard)",
        dsrr_preset_active_tag_strict: "Visual Layer (Strict)",
        dsrr_preset_active_desc: "Standard preset: Bypasses GameParam, Event, Script, animations, and map portals. DSRR provides DrawParam lighting, SFX, and textures.",
        dsrr_preset_active_desc_strict: "Strict preset: Overhaul mod completely overrides all shared files and map geometry/occlusion (CHR, SFX, Map MSB/FLVER/MCP, Param, Scripts, Menus). DSRR provides 100% pure 4K textures, 3D world objects (obj/), and armor/weapon models (parts/) with ZERO collisions or map tearing.",
        dsrr_preset_btn_disable: "Restore Full DSRR Files",
        mod_tag_visual_layer: "Visual Layer",
        mod_tag_visual_layer_strict: "Strict Visual",
        mod_disable_tooltip: "Click to disable this mod in scan and merge without deleting it",
        mod_enable_tooltip: "Click to enable this mod in scan and merge",
        badge_mod_disabled: "Disabled",
        alert_min_mods_active: "You need at least 2 active (checked) mods to run a conflict diagnostic."
    },
    pt: {
        app_title: "SoulsConflict",
        app_subtitle: "Verificador & Mesclador de Mods — Dark Souls / FromSoftware",
        open_mods_folder: "Abrir Pasta Mods",
        open_mods_folder_title: "Abrir pasta raiz onde ficam todos os mods",
        tab_merge: "Diagnóstico & Fusão",
        tab_launcher: "Launcher & Injeção DSR",
        status_vanilla: "Vanilla",
        status_modded: "Ativo",
        load_order_title: "Ordem de Carregamento",
        load_order_desc: "Adicione quantas pastas de mods quiser. A ordem define a prioridade de resolução: o mod no topo tem prioridade máxima.",
        btn_add_mod: "Adicionar Mod",
        btn_refresh: "Atualizar",
        btn_refresh_title: "Recarregar contagem de arquivos",
        btn_import_archive: "Importar Mod (.zip / .rar)",
        alert_err_not_archive: "Por favor, selecione ou arraste um arquivo de mod válido (.zip, .rar ou .7z).",
        uploading_archive_progress: "Extraindo e importando mod '{name}' nativamente...",
        output_folder_label: "Pasta de Saída:",
        btn_open_folder: "Abrir Pasta",
        btn_run_diagnostic: "Executar Diagnóstico",
        loading_title: "Analisando Mods...",
        loading_desc: "Descompactando DCX, cruzando tabelas BND3, IDs de texto FMG e parâmetros PARAM.",
        stat_analyzed: "Arquivos Analisados",
        stat_safe: "Totalmente Seguros",
        stat_mergeable: "Mescláveis",
        stat_conflict: "Conflitos Críticos",
        ready_merge_title: "Pronto para Fusão",
        evaluating_msg: "Avaliando compatibilidade dos mods ativos...",
        all_compatible_msg: "<strong style='color:var(--color-safe);'>100% Compatíveis.</strong> Todos os {total} mods podem ser unificados sem perda de dados.",
        conflicts_found_msg: "<strong>Atenção:</strong> {count} arquivo(s) com disputas de IDs entre os mods. Escolha a estratégia de resolução abaixo.",
        resolution_strategy_label: "Estratégia de Resolução:",
        mode_smart_title: "Fusão Inteligente",
        badge_recommended: "Recomendado",
        mode_smart_desc: "Detecta versões estendidas de scripts e entidades de mapas automaticamente.",
        mode_priority_title: "Prioridade Sequencial",
        mode_priority_desc: "Em caso de colisão, o mod no topo da lista vence.",
        mode_manual_title: "Escolha Manual",
        mode_manual_desc: "Permite escolher individualmente o mod vencedor para cada conflito.",
        btn_merge_all: "Mesclar Todos os Mods",
        tab_filter_all: "Todos",
        tab_filter_conflict: "Conflitos",
        tab_filter_mergeable: "Mescláveis",
        tab_filter_safe: "Seguros",
        launcher_title: "Integração Dark Souls: Remastered",
        launcher_desc: "Ative e jogue com a sua fusão de mods sem alterar permanentemente a pasta original do jogo. Alterne entre <strong>Modded</strong> e <strong>Vanilla</strong> em 1 clique.",
        checking_status: "Verificando...",
        exe_found: "Executável Localizado",
        exe_missing: "Executável Não Encontrado",
        banner_vanilla_title: "MODO VANILLA (ORIGINAL)",
        banner_vanilla_desc: "Nenhum arquivo modificado no jogo. Instalação original preservada.",
        banner_modded_title: "MODS ATIVOS",
        banner_modded_desc: "{count} arquivo(s) mesclados ativados. Originais em backup seguro.",
        banner_active_meta: "Ativado em: {time} · Backup em vanilla_backup/",
        banner_ready_meta: "{count} arquivo(s) prontos na pasta merged/ para aplicar.",
        active_mods_header: "Mods Ativos na Fusão (Ordem de Prioridade):",
        active_mods_desc: "O mod no topo (#1) tem a prioridade máxima em conflitos.",
        game_path_label: "Pasta de Instalação do Dark Souls Remastered:",
        game_path_hint: "Deve apontar para a pasta onde fica o arquivo <code>DarkSoulsRemastered.exe</code>.",
        btn_save: "Salvar",
        btn_save_title: "Salvar caminho",
        btn_open: "Abrir",
        btn_open_game_folder_title: "Abrir pasta do jogo no Explorer",
        btn_deploy_mods: "Ativar Mods no Jogo",
        btn_redeploy_mods: "Re-aplicar Mods",
        btn_restore_vanilla: "Restaurar Vanilla",
        btn_launch_game: "Launch DS1",
        btn_launch_seamless: "Launch DS1 Seamless Co-op",
        injected_files_title: "Arquivos Injetados",
        injected_files_desc: "Arquivos da fusão ativos na pasta do jogo com backup seguro.",
        no_injected_files: "Nenhum arquivo modificado ativo. O jogo está em modo Vanilla.",
        no_mods_configured: "Nenhum mod configurado. Vá na aba 'Diagnóstico & Fusão' para adicionar mods.",
        empty_mods_title: "Nenhum Mod Instalado na Pasta 'mods/'",
        empty_mods_desc: "O SoulsConflict é 100% portátil. Basta extrair seus mods baixados da Nexus Mods dentro da pasta <strong>mods/</strong>. Cada mod deve ficar em sua própria subpasta (ex: <code>mods/MeuMod/chr/...</code>).",
        btn_create_mod_folder: "Criar Pasta de Mod",
        badge_status_backup: "Com Backup",
        badge_status_new: "Novo",
        badge_ready: "{count} arqs",
        badge_empty: "Vazia",
        badge_auto_mapped: "Auto-Mapeado",
        badge_auto_mapped_title: "Arquivos fora de pastas padrão — auto-mapeado",
        btn_organize: "Organizar",
        btn_organize_title: "Organizar os arquivos desta pasta em suas pastas corretas",
        btn_delete_title: "Remover pasta",
        variant_label: "Variante:",
        variant_title: "Este mod possui variantes alternativas",
        prompt_new_folder: "Nome da pasta do Mod (ex: Cinders, Traducao, MeuMod):\nDeixe em branco para nome automático:",
        confirm_delete_folder: "Deseja remover a pasta 'mods/{id}'?\nOs arquivos dentro dela serão excluídos.",
        confirm_fix_structure: "Organizar automaticamente a estrutura de 'mods/{id}'?\nOs arquivos soltos serão movidos para as pastas canônicas (chr/, parts/, event/, etc.).",
        alert_min_mods: "Você precisa ter pelo menos 2 pastas de mods para diagnosticar conflitos.",
        alert_merge_success: "Mods mesclados com sucesso na pasta 'merged/'!",
        alert_restore_confirm: "Remover todos os mods ativos e restaurar os arquivos originais?",
        move_up_title: "Subir prioridade",
        move_down_title: "Descer prioridade",
        active_tag: "Ativo",
        staged_tag: "Preparado",
        modal_ok: "OK",
        modal_cancel: "Cancelar",
        modal_confirm: "Confirmar",
        modal_title_info: "Informação",
        modal_title_confirm: "Confirmação",
        modal_title_prompt: "Nova Pasta de Mod",
        modal_title_success: "Sucesso",
        modal_title_error: "Erro",
        alert_deploy_success: "Mods aplicados com sucesso! {placed} arquivo(s) ativados no Dark Souls ({backup} arquivos originais preservados em backup).",
        alert_restore_success: "Vanilla restaurado com sucesso! {removed} mod(s) removidos e {restored} arquivo(s) originais de fábrica restaurados.",
        alert_already_vanilla: "O jogo já está em estado Vanilla original (nenhum mod ativo).",
        alert_merge_result: "Fusão concluída com sucesso ({mode})!\n{copied} arquivos exclusivos copiados e {merged} arquivos em conflito unificados perfeitamente.",
        alert_fix_structure_success: "Estrutura corrigida com sucesso! {count} arquivo(s) organizados em pastas canônicas.",
        alert_game_path_saved: "Caminho do jogo salvo com sucesso.",
        alert_invalid_path: "Por favor insira um caminho válido para o Dark Souls Remastered.",
        alert_err_game_running: "O Dark Souls Remastered está em execução! Feche o jogo antes de continuar.",
        alert_err_exe_not_found: "DarkSoulsRemastered.exe não encontrado em: {path}",
        alert_err_merged_not_found: "A pasta 'merged/' não existe. Execute uma mesclagem na aba Diagnóstico primeiro!",
        alert_err_merged_empty: "A pasta 'merged/' está vazia! Mescle seus mods na aba Diagnóstico antes de aplicar.",
        alert_err_explorer: "Não foi possível abrir a pasta no Explorer.",
        alert_err_organize: "Falha ao organizar pasta.",
        alert_err_create_mod: "Falha ao criar pasta de mod.",
        alert_err_scan: "Erro no Diagnóstico: {msg}",
        alert_err_deploy: "Erro ao Aplicar: {msg}",
        alert_err_restore: "Erro ao Restaurar: {msg}",
        alert_err_launch: "Erro ao Iniciar: {msg}",
        alert_err_generic: "Ocorreu um erro: {msg}",
        filter_no_files: "Nenhum arquivo encontrado para este filtro.",
        launcher_status_active_session: "Sessão atual",
        btn_merging: "Mesclando...",
        details_discrepancies: "Detalhes & Discrepâncias Internas ({count}):",
        present_in: "Presente em",
        launch_mode_label: "Método de Inicialização:",
        launch_mode_auto: "Detecção Automática (Steam / Exe)",
        launch_mode_steam: "Steam (steam://run/570940)",
        launch_mode_direct: "Executável Direto (.exe)",
        alert_launch_steam_success: "Iniciando Dark Souls Remastered via Steam...",
        alert_launch_seamless_success: "Iniciando Seamless Co-op...",
        alert_launch_direct_success: "Iniciando Dark Souls Remastered diretamente via executável...",
        detected_steam_badge: "Steam Selecionada Automaticamente",
        detected_direct_badge: "Não-Steam / Alternativo Selecionado",
        mod_tag_active: "Ativo",
        mod_tag_overwritten: "Sobrescrito",
        mod_tag_combined: "Combinado",
        priority_winner_label: "Vencedor por Prioridade",
        overwritten_disabled_label: "Sobrescrito / Desativado",
        mergeable_base_label: "Arquivo Base (Prioridade #1)",
        mergeable_combined_label: "Combinado na Fusão",
        mergeable_banner_hint: "Fusão Inteligente: Alterações de ambos os mods são combinadas sem colisão ou perda de dados.",
        badge_resolved_toggle: "Resolvido via Toggle",
        badge_resolved_toggle_title: "Este conflito foi resolvido porque um mod secundário foi desativado (OFF) para este arquivo",
        conflict_choice_label: "Versão ativa do arquivo:",
        conflict_active_hint: "Marque ou desmarque um mod para escolher qual versão será incluída no merge.",
        custom_toggles_banner: "Seleções manuais ativas ({count} arquivos com toggle).",
        btn_restore_all_toggles: "Restaurar Prioridades Padrão",
        val_active_badge: "VENCEDOR",
        val_inactive_badge: "SOBRESCRITO",
        file_toggle_title_on: "Clique para desativar este arquivo neste mod",
        file_toggle_title_off: "Clique para reativar este arquivo neste mod",
        toggle_badge_off: "DESATIVADO",
        val_active_title: "Este valor vence e será usado no merge",
        val_inactive_title: "Este valor foi sobrescrito e será descartado",
        overwritten_mods_label: "Mods Sobrescritos",
        dsrr_preset_title: "Preset Camada Visual (Dark Souls Re-Remastered)",
        dsrr_preset_tag_detected: "DSRR Detectado",
        dsrr_preset_desc: "Escolha um preset automático para combinar o DSRR com mods overhaul sem travamentos na tela de loading.",
        dsrr_preset_btn_enable: "Preset Padrão (Lógica)",
        dsrr_preset_btn_enable_strict: "Preset Rígido (Zero Conflitos / Puro Gráfico)",
        dsrr_preset_active_title: "Camada Visual Ativa ({count} conflitos de lógica resolvidos)",
        dsrr_preset_active_title_strict: "Camada Visual Rígida Ativa ({count} arquivos ignorados no DSRR — Zero Conflitos)",
        dsrr_preset_active_tag: "Camada Visual (Padrão)",
        dsrr_preset_active_tag_strict: "Camada Visual (Rígida)",
        dsrr_preset_active_desc: "Preset padrão: O mod overhaul controla gameplay, missões, animações e geometria de mapa. O DSRR fornece texturas, iluminação (DrawParam) e SFX.",
        dsrr_preset_active_desc_strict: "Preset rígido: O mod overhaul sobrepõe 100% dos arquivos compartilhados e toda geometria/oclusão de mapa (CHR, SFX, Mapas MSB/FLVER/MCP, Param, Scripts, Menus). O DSRR atua exclusivamente como pacote 4K de texturas, armas/armaduras (parts/) e objetos de cenário (obj/) sem quebras de mapa ou conflitos.",
        dsrr_preset_btn_disable: "Restaurar Arquivos DSRR",
        mod_tag_visual_layer: "Camada Visual",
        mod_tag_visual_layer_strict: "Visual Rígido",
        mod_disable_tooltip: "Clique para desativar este mod no diagnóstico e fusão sem excluí-lo",
        mod_enable_tooltip: "Clique para ativar este mod no diagnóstico e fusão",
        badge_mod_disabled: "Desativado",
        alert_min_mods_active: "Você precisa de pelo menos 2 mods ativos (marcados) para executar o diagnóstico."
    },
    it: {
        app_title: "SoulsConflict",
        app_subtitle: "Verificatore e Fusione di Mod — Dark Souls / FromSoftware",
        open_mods_folder: "Apri Cartella Mod",
        open_mods_folder_title: "Apri la cartella principale in cui sono conservate tutte le mod",
        tab_merge: "Diagnostica & Fusione",
        tab_launcher: "Launcher & Iniezione DSR",
        status_vanilla: "Vanilla",
        status_modded: "Attivo",
        load_order_title: "Ordine di Caricamento",
        load_order_desc: "Aggiungi tutte le cartelle mod desiderate. L'ordine definisce la priorità: la mod in cima ha la priorità più alta.",
        btn_add_mod: "Aggiungi Mod",
        btn_refresh: "Aggiorna",
        btn_refresh_title: "Ricarica il conteggio dei file",
        btn_import_archive: "Importa Mod (.zip / .rar)",
        alert_err_not_archive: "Seleziona o trascina un archivio mod valido (.zip, .rar o .7z).",
        uploading_archive_progress: "Estrazione e importazione mod '{name}' nativamente...",
        output_folder_label: "Cartella di Uscita:",
        btn_open_folder: "Apri Cartella",
        btn_run_diagnostic: "Esegui Diagnostica",
        loading_title: "Analisi delle Mod in corso...",
        loading_desc: "Decompressione DCX, confronto tabelle BND3, ID testo FMG e parametri PARAM.",
        stat_analyzed: "File Analizzati",
        stat_safe: "Completamente Sicuri",
        stat_mergeable: "Fusibili",
        stat_conflict: "Conflitti Critici",
        ready_merge_title: "Pronto per la Fusione",
        evaluating_msg: "Valutazione compatibilità delle mod attive...",
        all_compatible_msg: "<strong style='color:var(--color-safe);'>100% Compatibili.</strong> Tutte le {total} mod possono essere unite senza perdita di dati.",
        conflicts_found_msg: "<strong>Attenzione:</strong> {count} file con collisioni di ID tra le mod. Scegli una strategia di risoluzione qui sotto.",
        resolution_strategy_label: "Strategia di Risoluzione:",
        mode_smart_title: "Fusione Intelligente",
        badge_recommended: "Consigliato",
        mode_smart_desc: "Rileva automaticamente versioni estese di script ed entità di mappa.",
        mode_priority_title: "Priorità Sequenziale",
        mode_priority_desc: "In caso di collisione, vince la mod posizionata più in alto nell'elenco.",
        mode_manual_title: "Scelta Manuale",
        mode_manual_desc: "Consente di scegliere individualmente la mod vincente per ogni conflitto.",
        btn_merge_all: "Unisci Tutte le Mod",
        tab_filter_all: "Tutti",
        tab_filter_conflict: "Conflitti",
        tab_filter_mergeable: "Fusibili",
        tab_filter_safe: "Sicuri",
        launcher_title: "Integrazione Dark Souls: Remastered",
        launcher_desc: "Attiva e gioca con la tua fusione di mod senza modificare permanentemente i file originali. Passa da <strong>Modded</strong> a <strong>Vanilla</strong> con 1 clic.",
        checking_status: "Controllo in corso...",
        exe_found: "Eseguibile Trovato",
        exe_missing: "Eseguibile Non Trovato",
        banner_vanilla_title: "MODALITÀ VANILLA (ORIGINALE)",
        banner_vanilla_desc: "Nessun file modificato. Installazione di fabbrica preservata.",
        banner_modded_title: "MOD ATTIVE",
        banner_modded_desc: "{count} file uniti attivati. Backup degli originali al sicuro.",
        banner_active_meta: "Attivato alle: {time} · Backup in vanilla_backup/",
        banner_ready_meta: "{count} file pronti nella cartella merged/ da applicare.",
        active_mods_header: "Mod Attive nella Fusione (Ordine di Caricamento):",
        active_mods_desc: "La mod in cima (#1) ha la priorità massima nei conflitti.",
        game_path_label: "Cartella di Installazione di Dark Souls Remastered:",
        game_path_hint: "Deve puntare alla cartella contenente <code>DarkSoulsRemastered.exe</code>.",
        btn_save: "Salva",
        btn_save_title: "Salva percorso",
        btn_open: "Apri",
        btn_open_game_folder_title: "Apri la cartella del gioco in Explorer",
        btn_deploy_mods: "Attiva Mod nel Gioco",
        btn_redeploy_mods: "Riapplica Mod",
        btn_restore_vanilla: "Ripristina Vanilla",
        btn_launch_game: "Avvia Dark Souls Remastered",
        injected_files_title: "File Iniettati",
        injected_files_desc: "File uniti attivi nella cartella del gioco con backup sicuro.",
        no_injected_files: "Nessun file modificato attivo. Il gioco è in modalità Vanilla.",
        no_mods_configured: "Nessuna mod configurata. Vai alla scheda 'Diagnostica & Fusione' per aggiungere mod.",
        empty_mods_title: "Nessuna Mod Trovata nella Cartella 'mods/'",
        empty_mods_desc: "SoulsConflict è portatile al 100%. Estrai le mod scaricate all'interno della cartella <strong>mods/</strong>. Ogni mod deve risiedere nella propria sottocartella.",
        btn_create_mod_folder: "Crea Cartella Mod",
        badge_status_backup: "Con Backup",
        badge_status_new: "Nuovo",
        badge_ready: "{count} file",
        badge_empty: "Vuota",
        badge_auto_mapped: "Auto-Mappato",
        badge_auto_mapped_title: "File esterni alle cartelle standard — mappati automaticamente",
        btn_organize: "Organizza",
        btn_organize_title: "Organizza i file sciolti nelle rispettive cartelle canoniche",
        btn_delete_title: "Elimina cartella",
        variant_label: "Variante:",
        variant_title: "Questa mod presenta varianti alternative",
        prompt_new_folder: "Nome della cartella Mod (es. Cinders, BetterRolling, MiaMod):\nLascia vuoto per nome automatico:",
        confirm_delete_folder: "Rimuovere la cartella 'mods/{id}'?\nTutti i file all'interno verranno eliminati.",
        confirm_fix_structure: "Organizzare automaticamente la struttura di 'mods/{id}'?\nI file sciolti verranno spostati nelle cartelle canoniche (chr/, param/, event/, ecc.).",
        alert_min_mods: "Sono necessarie almeno 2 cartelle mod per eseguire la diagnostica.",
        alert_merge_success: "Mod unite con successo nella cartella 'merged/'!",
        alert_restore_confirm: "Rimuovere tutte le mod attive e ripristinare i file originali?",
        move_up_title: "Aumenta priorità",
        move_down_title: "Diminuisci priorità",
        active_tag: "Attivo",
        staged_tag: "Pronto",
        modal_ok: "OK",
        modal_cancel: "Annulla",
        modal_confirm: "Conferma",
        modal_title_info: "Informazione",
        modal_title_confirm: "Conferma",
        modal_title_prompt: "Nuova Cartella Mod",
        modal_title_success: "Successo",
        modal_title_error: "Errore",
        alert_deploy_success: "Mod applicate con successo! {placed} file attivati in Dark Souls ({backup} file originali preservati in backup).",
        alert_restore_success: "Vanilla ripristinato con successo! {removed} file mod rimossi e {restored} file originali ripristinati.",
        alert_already_vanilla: "Il gioco è già nello stato Vanilla originale (nessuna mod attiva).",
        alert_merge_result: "Fusione completata con successo ({mode})!\n{copied} file esclusivi copiati e {merged} file in conflitto unificati.",
        alert_fix_structure_success: "Struttura organizzata con successo! {count} file posizionati nelle cartelle canoniche.",
        alert_game_path_saved: "Percorso del gioco salvato con successo.",
        alert_invalid_path: "Inserisci un percorso valido per Dark Souls Remastered.",
        alert_err_game_running: "Dark Souls Remastered è attualmente in esecuzione! Chiudi il gioco prima di procedere.",
        alert_err_exe_not_found: "DarkSoulsRemastered.exe non è stato trovato in: {path}",
        alert_err_merged_not_found: "La cartella 'merged/' non esiste. Esegui prima una fusione nella scheda Diagnostica!",
        alert_err_merged_empty: "La cartella 'merged/' è vuota! Unisci le tue mod nella scheda Diagnostica prima di applicare.",
        alert_err_explorer: "Impossibile aprire la cartella in Explorer.",
        alert_err_organize: "Impossibile organizzare la cartella.",
        alert_err_create_mod: "Impossibile creare la cartella mod.",
        alert_err_scan: "Errore Diagnostica: {msg}",
        alert_err_deploy: "Errore Attivazione: {msg}",
        alert_err_restore: "Errore Ripristino: {msg}",
        alert_err_launch: "Errore Avvio: {msg}",
        alert_err_generic: "Si è verificato un errore: {msg}",
        filter_no_files: "Nessun file trovato per questo filtro.",
        launcher_status_active_session: "Sessione corrente",
        btn_merging: "Fusione in corso...",
        details_discrepancies: "Dettagli & Discrepanze Interne ({count}):",
        present_in: "Presente in",
        launch_mode_label: "Metodo di Avvio:",
        launch_mode_auto: "Rilevamento Automatico (Steam / Exe)",
        launch_mode_steam: "Steam (steam://run/570940)",
        launch_mode_direct: "Eseguibile Diretto (.exe)",
        alert_launch_steam_success: "Avvio di Dark Souls Remastered tramite Steam in corso...",
        alert_launch_direct_success: "Avvio di Dark Souls Remastered direttamente tramite eseguibile...",
        detected_steam_badge: "Steam Selezionato Automaticamente",
        detected_direct_badge: "Non-Steam / Alternativo Selezionato"
    },
    fr: {
        app_title: "SoulsConflict",
        app_subtitle: "Vérificateur et Fusionneur de Mods — Dark Souls / FromSoftware",
        open_mods_folder: "Ouvrir Dossier Mods",
        open_mods_folder_title: "Ouvrir le dossier racine contenant tous les mods",
        tab_merge: "Diagnostic & Fusion",
        tab_launcher: "Lanceur & Injection DSR",
        status_vanilla: "Vanilla",
        status_modded: "Actif",
        load_order_title: "Ordre de Chargement",
        load_order_desc: "Ajoutez autant de dossiers de mods que vous le souhaitez. L'ordre définit la priorité : le mod en haut est prioritaire.",
        btn_add_mod: "Ajouter Mod",
        btn_refresh: "Actualiser",
        btn_refresh_title: "Actualiser le décompte des fichiers",
        btn_import_archive: "Importer un Mod (.zip / .rar)",
        alert_err_not_archive: "Veuillez sélectionner ou déposer une archive de mod valide (.zip, .rar ou .7z).",
        uploading_archive_progress: "Extraction et importation du mod '{name}'...",
        output_folder_label: "Dossier de Sortie :",
        btn_open_folder: "Ouvrir Dossier",
        btn_run_diagnostic: "Lancer le Diagnostic",
        loading_title: "Analyse des Mods...",
        loading_desc: "Décompression DCX, correspondance des tables BND3, ID de texte FMG et paramètres PARAM.",
        stat_analyzed: "Fichiers Analysés",
        stat_safe: "Parfaitement Sûrs",
        stat_mergeable: "Fusionnables",
        stat_conflict: "Conflits Critiques",
        ready_merge_title: "Prêt pour la Fusion",
        evaluating_msg: "Évaluation de la compatibilité des mods actifs...",
        all_compatible_msg: "<strong style='color:var(--color-safe);'>100% Compatibles.</strong> Tous les {total} mods peuvent être fusionnés sans perte de données.",
        conflicts_found_msg: "<strong>Attention :</strong> {count} fichier(s) avec des conflits d'ID. Choisissez une stratégie de résolution ci-dessous.",
        resolution_strategy_label: "Stratégie de Résolution :",
        mode_smart_title: "Fusion Intelligente",
        badge_recommended: "Recommandé",
        mode_smart_desc: "Détecte automatiquement les versions étendues des scripts et des entités de cartes.",
        mode_priority_title: "Priorité Séquentielle",
        mode_priority_desc: "En cas de collision, le mod le plus haut dans la liste l'emporte.",
        mode_manual_title: "Choix Manuel",
        mode_manual_desc: "Permet de sélectionner individuellement le mod gagnant pour chaque conflit.",
        btn_merge_all: "Fusionner Tous les Mods",
        tab_filter_all: "Tous",
        tab_filter_conflict: "Conflits",
        tab_filter_mergeable: "Fusionnables",
        tab_filter_safe: "Sûrs",
        launcher_title: "Intégration Dark Souls: Remastered",
        launcher_desc: "Activez et jouez à votre fusion de mods sans altérer définitivement les fichiers originaux. Basculez entre <strong>Moddé</strong> et <strong>Vanilla</strong> en 1 clic.",
        checking_status: "Vérification...",
        exe_found: "Exécutable Localisé",
        exe_missing: "Exécutable Non Trouvé",
        banner_vanilla_title: "MODE VANILLA (ORIGINAL)",
        banner_vanilla_desc: "Aucun fichier modifié. Installation d'usine préservée.",
        banner_modded_title: "MODS ACTIFS",
        banner_modded_desc: "{count} fichier(s) fusionné(s) activé(s). Sauvegarde des originaux sécurisée.",
        banner_active_meta: "Activé à : {time} · Sauvegarde dans vanilla_backup/",
        banner_ready_meta: "{count} fichier(s) prêts dans le dossier merged/ à déployer.",
        active_mods_header: "Mods Actifs dans la Fusion (Ordre de Priorité) :",
        active_mods_desc: "Le mod en haut (#1) a la priorité absolue en cas de conflit.",
        game_path_label: "Dossier d'Installation de Dark Souls Remastered :",
        game_path_hint: "Doit pointer vers le dossier contenant <code>DarkSoulsRemastered.exe</code>.",
        btn_save: "Enregistrer",
        btn_save_title: "Enregistrer le chemin",
        btn_open: "Ouvrir",
        btn_open_game_folder_title: "Ouvrir le dossier du jeu dans l'Explorateur",
        btn_deploy_mods: "Activer les Mods dans le Jeu",
        btn_redeploy_mods: "Réappliquer les Mods",
        btn_restore_vanilla: "Restaurer Vanilla",
        btn_launch_game: "Lancer Dark Souls Remastered",
        injected_files_title: "Fichiers Injectés",
        injected_files_desc: "Fichiers fusionnés actifs dans le jeu avec sauvegarde sécurisée.",
        no_injected_files: "Aucun fichier modifié actif. Le jeu est en mode Vanilla.",
        no_mods_configured: "Aucun mod configuré. Rendez-vous dans l'onglet 'Diagnostic & Fusion' pour ajouter des mods.",
        empty_mods_title: "Aucun Mod Trouvé dans le Dossier 'mods/'",
        empty_mods_desc: "SoulsConflict est 100% portable. Extrayez simplement vos mods téléchargés dans le dossier <strong>mods/</strong>. Chaque mod doit être dans son propre sous-dossier.",
        btn_create_mod_folder: "Créer Dossier Mod",
        badge_status_backup: "Sauvegardé",
        badge_status_new: "Nouveau",
        badge_ready: "{count} fichiers",
        badge_empty: "Vide",
        badge_auto_mapped: "Auto-Mappé",
        badge_auto_mapped_title: "Fichiers hors répertoires standards — automatiquement mappés",
        btn_organize: "Organiser",
        btn_organize_title: "Ranger les fichiers libres dans leurs répertoires standards",
        btn_delete_title: "Supprimer le dossier",
        variant_label: "Variante :",
        variant_title: "Ce mod propose des variantes alternatives",
        prompt_new_folder: "Nom du dossier du mod (ex. Cinders, BetterRolling, MonMod) :\nLaissez vide pour un nom automatique :",
        confirm_delete_folder: "Supprimer le dossier 'mods/{id}' ?\nTous les fichiers qu'il contient seront effacés.",
        confirm_fix_structure: "Organiser automatiquement la structure de 'mods/{id}' ?\nLes fichiers libres seront déplacés vers les répertoires canoniques (chr/, param/, event/, etc.).",
        alert_min_mods: "Vous devez avoir au moins 2 dossiers de mods pour exécuter un diagnostic.",
        alert_merge_success: "Mods fusionnés avec succès dans le dossier 'merged/' !",
        alert_restore_confirm: "Supprimer tous les mods actifs et restaurer les fichiers originaux ?",
        move_up_title: "Monter la priorité",
        move_down_title: "Descendre la priorité",
        active_tag: "Actif",
        staged_tag: "Préparé",
        modal_ok: "OK",
        modal_cancel: "Annuler",
        modal_confirm: "Confirmer",
        modal_title_info: "Information",
        modal_title_confirm: "Confirmation",
        modal_title_prompt: "Nouveau Dossier de Mod",
        modal_title_success: "Succès",
        modal_title_error: "Erreur",
        alert_deploy_success: "Mods appliqués avec succès ! {placed} fichier(s) activé(s) dans Dark Souls ({backup} fichiers originaux sauvegardés).",
        alert_restore_success: "Vanilla restauré avec succès ! {removed} fichier(s) de mod supprimé(s) et {restored} fichier(s) originaux restaurés.",
        alert_already_vanilla: "Le jeu est déjà dans son état Vanilla original (aucun mod actif).",
        alert_merge_result: "Fusion terminée avec succès ({mode}) !\n{copied} fichier(s) unique(s) copié(s) et {merged} fichier(s) en conflit unifié(s).",
        alert_fix_structure_success: "Structure organisée avec succès ! {count} fichier(s) placés dans les dossiers canoniques.",
        alert_game_path_saved: "Chemin du jeu enregistré avec succès.",
        alert_invalid_path: "Veuillez saisir un chemin valide pour Dark Souls Remastered.",
        alert_err_game_running: "Dark Souls Remastered est en cours d'exécution ! Veuillez fermer le jeu avant de continuer.",
        alert_err_exe_not_found: "DarkSoulsRemastered.exe n'a pas été trouvé dans : {path}",
        alert_err_merged_not_found: "Le dossier 'merged/' n'existe pas. Lancez d'abord une fusion dans l'onglet Diagnostic !",
        alert_err_merged_empty: "Le dossier 'merged/' est vide ! Fusionnez vos mods dans l'onglet Diagnostic avant de les appliquer.",
        alert_err_explorer: "Impossible d'ouvrir le dossier dans l'Explorateur.",
        alert_err_organize: "Échec de l'organisation du dossier.",
        alert_err_create_mod: "Échec de la création du dossier de mod.",
        alert_err_scan: "Erreur de Diagnostic : {msg}",
        alert_err_deploy: "Erreur de Déploiement : {msg}",
        alert_err_restore: "Erreur de Restauration : {msg}",
        alert_err_launch: "Erreur de Lancement : {msg}",
        alert_err_generic: "Une erreur est survenue : {msg}",
        filter_no_files: "Aucun fichier trouvé pour ce filtre.",
        launcher_status_active_session: "Session actuelle",
        btn_merging: "Fusion en cours...",
        details_discrepancies: "Détails & Écarts Internes ({count}) :",
        present_in: "Présent dans",
        launch_mode_label: "Méthode de Lancement :",
        launch_mode_auto: "Détection Automatique (Steam / Exe)",
        launch_mode_steam: "Steam (steam://run/570940)",
        launch_mode_direct: "Exécutable Direct (.exe)",
        alert_launch_steam_success: "Lancement de Dark Souls Remastered via Steam...",
        alert_launch_direct_success: "Lancement de Dark Souls Remastered directement via l'exécutable...",
        detected_steam_badge: "Steam Sélectionné Automatiquement",
        detected_direct_badge: "Non-Steam / Alternatif Sélectionné"
    },
    ja: {
        app_title: "SoulsConflict",
        app_subtitle: "Mod競合検出・統合ツール — ダークソウル / フロム・ソフトウェア",
        open_mods_folder: "Modsフォルダを開く",
        open_mods_folder_title: "すべてのModが格納されているルートフォルダを開く",
        tab_merge: "診断と統合",
        tab_launcher: "ランチャー & DSR適用",
        status_vanilla: "バニラ",
        status_modded: "適用中",
        load_order_title: "ロード順序",
        load_order_desc: "任意の数のModフォルダを追加できます。順序によって優先度が決まります。一番上のModが最優先されます。",
        btn_add_mod: "Modを追加",
        btn_refresh: "更新",
        btn_refresh_title: "ファイル数を再読み込み",
        btn_import_archive: "Modをインポート (.zip / .rar)",
        alert_err_not_archive: "有効なModアーカイブ (.zip, .rar, .7z) を選択またはドロップしてください。",
        uploading_archive_progress: "Mod '{name}' をネイティブ展開してインポート中...",
        output_folder_label: "出力フォルダ:",
        btn_open_folder: "フォルダを開く",
        btn_run_diagnostic: "診断を実行",
        loading_title: "Modを分析中...",
        loading_desc: "DCXの解凍、BND3テーブル、FMGテキストID、PARAMパラメータを照合中。",
        stat_analyzed: "分析されたファイル",
        stat_safe: "完全に安全",
        stat_mergeable: "統合可能",
        stat_conflict: "競合発生",
        ready_merge_title: "統合の準備完了",
        evaluating_msg: "アクティブなModの互換性を評価中...",
        all_compatible_msg: "<strong style='color:var(--color-safe);'>100% 互換性あり。</strong> すべての {total} Modがデータ損失なく統合可能です。",
        conflicts_found_msg: "<strong>注意:</strong> {count} 個のファイルでIDの競合が検出されました。以下の解決戦略を選択してください。",
        resolution_strategy_label: "解決戦略:",
        mode_smart_title: "スマート統合",
        badge_recommended: "推奨",
        mode_smart_desc: "スクリプトやマップエンティティの拡張バージョンを自動検出します。",
        mode_priority_title: "順序優先",
        mode_priority_desc: "競合が発生した場合、リストの上位にあるModが優先されます。",
        mode_manual_title: "手動選択",
        mode_manual_desc: "競合ごとに個別に優先するModを選択できます。",
        btn_merge_all: "すべてのModを統合",
        tab_filter_all: "すべて",
        tab_filter_conflict: "競合",
        tab_filter_mergeable: "統合可能",
        tab_filter_safe: "安全",
        launcher_title: "Dark Souls: Remastered 連携",
        launcher_desc: "元のゲームファイルを恒久的に変更することなく、統合Modを適用してプレイできます。1クリックで<strong>Mod適用</strong>と<strong>バニラ</strong>を切り替えられます。",
        checking_status: "確認中...",
        exe_found: "実行ファイル検出",
        exe_missing: "実行ファイルが見つかりません",
        banner_vanilla_title: "バニラモード (オリジナル)",
        banner_vanilla_desc: "変更されたファイルはありません。元のゲームファイルが保護されています。",
        banner_modded_title: "Mod適用中",
        banner_modded_desc: "{count} 個の統合ファイルがゲーム内で有効です。元のファイルは安全にバックアップされています。",
        banner_active_meta: "適用時刻: {time} · バックアップ先: vanilla_backup/",
        banner_ready_meta: "{count} 個のファイルが merged/ フォルダで適用待機中。",
        active_mods_header: "統合内のアクティブMod (ロード順序):",
        active_mods_desc: "最上位 (#1) のModが競合時に最優先されます。",
        game_path_label: "Dark Souls Remastered インストール先フォルダ:",
        game_path_hint: "<code>DarkSoulsRemastered.exe</code> が存在するフォルダを指定してください。",
        btn_save: "保存",
        btn_save_title: "パスを保存",
        btn_open: "開く",
        btn_open_game_folder_title: "エクスプローラーでゲームフォルダを開く",
        btn_deploy_mods: "ゲームにModを適用",
        btn_redeploy_mods: "Modを再適用",
        btn_restore_vanilla: "バニラに戻す",
        btn_launch_game: "Dark Souls Remasteredを起動",
        injected_files_title: "適用されたファイル",
        injected_files_desc: "ゲームフォルダに適用中の統合ファイル (バックアップ保持)。",
        no_injected_files: "変更されたファイルはありません。ゲームはバニラ状態です。",
        no_mods_configured: "Modが設定されていません。「診断と統合」タブでModを追加してください。",
        empty_mods_title: "mods/ フォルダにModが見つかりません",
        empty_mods_desc: "SoulsConflictは完全ポータブルです。ダウンロードしたModを <strong>mods/</strong> フォルダ内に展開してください。各Modは個別のサブフォルダに格納する必要があります (例: <code>mods/MyMod/chr/...</code>)。",
        btn_create_mod_folder: "Modフォルダを作成",
        badge_status_backup: "退避済",
        badge_status_new: "新規",
        badge_ready: "{count} 個",
        badge_empty: "空",
        badge_auto_mapped: "自動配置",
        badge_auto_mapped_title: "標準外フォルダのファイル — 自動マッピング",
        btn_organize: "整理",
        btn_organize_title: "このフォルダ内のファイルを標準フォルダ構造に整理する",
        btn_delete_title: "フォルダを削除",
        variant_label: "バリアント:",
        variant_title: "このModには複数の選択肢・バリアントが存在します",
        prompt_new_folder: "Modフォルダ名 (例: Cinders, BetterRolling, MyMod):\n空白のままにすると自動採番されます:",
        confirm_delete_folder: "フォルダ 'mods/{id}' を削除しますか？\n内部のすべてのファイルが削除されます。",
        confirm_fix_structure: "'mods/{id}' の構造を自動整理しますか？\n直下のファイルが正規フォルダ (chr/, param/, event/ 等) へ移動されます。",
        alert_min_mods: "競合診断には最低2つ以上のModフォルダが必要です。",
        alert_merge_success: "Modが 'merged/' フォルダに正常に統合されました！",
        alert_restore_confirm: "適用中のすべてのModを解除し、元のゲームファイルに戻しますか？",
        move_up_title: "優先度を上げる",
        move_down_title: "優先度を下げる",
        active_tag: "有効",
        staged_tag: "待機中",
        modal_ok: "OK",
        modal_cancel: "キャンセル",
        modal_confirm: "確認",
        modal_title_info: "お知らせ",
        modal_title_confirm: "確認",
        modal_title_prompt: "新規Modフォルダ",
        modal_title_success: "完了",
        modal_title_error: "エラー",
        alert_deploy_success: "Modの適用に成功しました！{placed}個のファイルをDark Soulsで有効化しました（{backup}個の元ファイルを安全にバックアップ）。",
        alert_restore_success: "バニラ状態を正常に復元しました！{removed}個のModファイルを削除し、{restored}個の元ファイルを復元しました。",
        alert_already_vanilla: "ゲームは既にオリジナルのバニラ状態です（有効なModはありません）。",
        alert_merge_result: "結合が正常に完了しました（{mode}）！\n{copied}個の固有ファイルをコピーし、{merged}個の競合ファイルを統合しました。",
        alert_fix_structure_success: "フォルダー構造を正常に整理しました！{count}個のファイルを正規フォルダーに配置しました。",
        alert_game_path_saved: "ゲームのパスが正常に保存されました。",
        alert_invalid_path: "有効なDark Souls Remasteredのインストールパスを入力してください。",
        alert_err_game_running: "Dark Souls Remasteredが起動中です！続行する前にゲームを終了してください。",
        alert_err_exe_not_found: "DarkSoulsRemastered.exe が見つかりませんでした: {path}",
        alert_err_merged_not_found: "「merged/」フォルダーが存在しません。先に「診断と結合」タブで結合を実行してください！",
        alert_err_merged_empty: "「merged/」フォルダーが空です！適用する前に「診断と結合」タブでModを結合してください。",
        alert_err_explorer: "エクスプローラーでフォルダーを開けませんでした。",
        alert_err_organize: "フォルダーの整理に失敗しました。",
        alert_err_create_mod: "Modフォルダーの作成に失敗しました。",
        alert_err_scan: "診断エラー: {msg}",
        alert_err_deploy: "適用エラー: {msg}",
        alert_err_restore: "復元エラー: {msg}",
        alert_err_launch: "起動エラー: {msg}",
        alert_err_generic: "エラーが発生しました: {msg}",
        filter_no_files: "このフィルターに一致するファイルは見つかりませんでした。",
        launcher_status_active_session: "現在のセッション",
        btn_merging: "結合中...",
        details_discrepancies: "詳細および内部の不一致（{count}件）:",
        present_in: "含まれるMod",
        launch_mode_label: "起動方法:",
        launch_mode_auto: "自動検出 (Steam / Exe)",
        launch_mode_steam: "Steam (steam://run/570940)",
        launch_mode_direct: "直接実行 (.exe)",
        alert_launch_steam_success: "Steam経由でDark Souls Remasteredを起動しています...",
        alert_launch_direct_success: "実行ファイルから直接Dark Souls Remasteredを起動しています...",
        detected_steam_badge: "Steam自動選択",
        detected_direct_badge: "非Steam / 代替版自動選択"
    },
    zh: {
        app_title: "SoulsConflict",
        app_subtitle: "模组冲突检测与合并工具 — 黑暗之魂 / FromSoftware",
        open_mods_folder: "打开模组文件夹",
        open_mods_folder_title: "打开存放所有模组的根目录",
        tab_merge: "诊断与合并",
        tab_launcher: "启动器与DSR注入",
        status_vanilla: "原版",
        status_modded: "已激活",
        load_order_title: "加载顺序",
        load_order_desc: "可以添加任意数量的模组文件夹。列表顺序决定解决优先级：顶部的模组具有最高优先级。",
        btn_add_mod: "添加模组",
        btn_refresh: "刷新",
        btn_refresh_title: "重新加载文件计数",
        btn_import_archive: "导入模组 (.zip / .rar)",
        alert_err_not_archive: "请选择或拖放有效的模组压缩包 (.zip, .rar 或 .7z)。",
        uploading_archive_progress: "正在解压并导入模组 '{name}'...",
        output_folder_label: "输出目录:",
        btn_open_folder: "打开文件夹",
        btn_run_diagnostic: "执行诊断",
        loading_title: "正在分析模组...",
        loading_desc: "解压缩 DCX，比对 BND3 数据包、FMG 文本 ID 以及 PARAM 参数表。",
        stat_analyzed: "已分析文件",
        stat_safe: "完全安全",
        stat_mergeable: "可合并",
        stat_conflict: "严重冲突",
        ready_merge_title: "准备合并",
        evaluating_msg: "正在评估当前模组的兼容性...",
        all_compatible_msg: "<strong style='color:var(--color-safe);'>100% 兼容。</strong> 所有 {total} 个模组均可合并，无数据丢失。",
        conflicts_found_msg: "<strong>注意:</strong> 检测到 {count} 个文件存在 ID 冲突。请在下方选择解决策略。",
        resolution_strategy_label: "解决策略:",
        mode_smart_title: "智能合并",
        badge_recommended: "推荐",
        mode_smart_desc: "自动检测脚本扩展版本和地图实体数据。",
        mode_priority_title: "顺序优先",
        mode_priority_desc: "遇到冲突时，加载顺序靠前的模组胜出。",
        mode_manual_title: "手动选择",
        mode_manual_desc: "允许为每个冲突文件单独选择优先采用的模组。",
        btn_merge_all: "合并所有模组",
        tab_filter_all: "全部",
        tab_filter_conflict: "冲突",
        tab_filter_mergeable: "可合并",
        tab_filter_safe: "安全",
        launcher_title: "Dark Souls: Remastered 整合与管理",
        launcher_desc: "启用并畅玩合并后的模组，无需永久覆盖原始游戏文件。一键轻松切换 <strong>Modded</strong> 和 <strong>Vanilla (原版)</strong>。",
        checking_status: "正在检测...",
        exe_found: "已定位可执行文件",
        exe_missing: "未找到游戏可执行文件",
        banner_vanilla_title: "原版模式 (VANILLA)",
        banner_vanilla_desc: "未修改任何游戏文件。原始安装完好无损。",
        banner_modded_title: "模组已激活",
        banner_modded_desc: "已激活 {count} 个合并文件。原版文件安全备份在案。",
        banner_active_meta: "激活时间: {time} · 备份保存于 vanilla_backup/",
        banner_ready_meta: "merged/ 目录中有 {count} 个文件等待注入。",
        active_mods_header: "合并中的活动模组 (加载顺序):",
        active_mods_desc: "位于顶部的模组 (#1) 具有最高冲突优先权。",
        game_path_label: "Dark Souls Remastered 安装路径:",
        game_path_hint: "必须指向包含 <code>DarkSoulsRemastered.exe</code> 的游戏主目录。",
        btn_save: "保存",
        btn_save_title: "保存路径",
        btn_open: "打开",
        btn_open_game_folder_title: "在文件资源管理器中打开游戏目录",
        btn_deploy_mods: "在游戏中启用模组",
        btn_redeploy_mods: "重新部署模组",
        btn_restore_vanilla: "恢复原版",
        btn_launch_game: "启动 Dark Souls Remastered",
        injected_files_title: "已注入文件",
        injected_files_desc: "当前在游戏目录中生效的合并文件 (带安全备份)。",
        no_injected_files: "当前无生效修改文件。游戏处于原版状态。",
        no_mods_configured: "未配置任何模组。请前往“诊断与合并”选项卡添加模组。",
        empty_mods_title: "mods 文件夹中未发现模组",
        empty_mods_desc: "SoulsConflict 完全绿色便携。请将从 Nexus 下载的模组解压到 <strong>mods/</strong> 目录内。每个模组应包含在独立子文件夹中 (例如: <code>mods/MyMod/chr/...</code>)。",
        btn_create_mod_folder: "创建模组文件夹",
        badge_status_backup: "已备份原版",
        badge_status_new: "新增",
        badge_ready: "{count} 个文件",
        badge_empty: "空",
        badge_auto_mapped: "自动映射",
        badge_auto_mapped_title: "位于非标准目录的文件 — 已自动适配",
        btn_organize: "整理",
        btn_organize_title: "将此文件夹内的松散文件整理到标准规范目录中",
        btn_delete_title: "删除文件夹",
        variant_label: "变体:",
        variant_title: "此模组包含可选分支/变体",
        prompt_new_folder: "模组文件夹名称 (如 Cinders, BetterRolling, MyMod):\n留空则使用默认自动名称:",
        confirm_delete_folder: "确定删除文件夹 'mods/{id}' 吗？\n其中的所有文件将被彻底删除。",
        confirm_fix_structure: "确定自动整理 'mods/{id}' 的目录结构吗？\n根目录散落文件将被归类至对应目录 (chr/, param/, event/ 等)。",
        alert_min_mods: "需要至少 2 个模组文件夹才能执行冲突诊断。",
        alert_merge_success: "模组已成功合并至 'merged/' 目录！",
        alert_restore_confirm: "确定要移除所有已激活的模组并恢复原厂游戏文件吗？",
        move_up_title: "提高优先级",
        move_down_title: "降低优先级",
        active_tag: "已激活",
        staged_tag: "就绪",
        modal_ok: "确定",
        modal_cancel: "取消",
        modal_confirm: "确认",
        modal_title_info: "提示",
        modal_title_confirm: "确认操作",
        modal_title_prompt: "新建模组文件夹",
        modal_title_success: "成功",
        modal_title_error: "错误",
        alert_deploy_success: "Mod应用成功！{placed} 个文件已在Dark Souls中生效（{backup} 个原始文件已安全备份）。",
        alert_restore_success: "原版恢复成功！已移除 {removed} 个Mod文件并恢复 {restored} 个原厂文件。",
        alert_already_vanilla: "游戏已处于原始原版状态（无已生效Mod）。",
        alert_merge_result: "合并圆满完成（{mode}）！\n已复制 {copied} 个独立文件，并整合 {merged} 个冲突文件。",
        alert_fix_structure_success: "结构整理成功！已将 {count} 个文件归纳到规范目录中。",
        alert_game_path_saved: "游戏路径已成功保存。",
        alert_invalid_path: "请输入有效的 Dark Souls Remastered 游戏安装目录。",
        alert_err_game_running: "Dark Souls Remastered 正在运行中！请先关闭游戏后再继续。",
        alert_err_exe_not_found: "在以下路径未找到 DarkSoulsRemastered.exe：{path}",
        alert_err_merged_not_found: "“merged/”文件夹不存在。请先在“诊断与合并”选项卡中执行合并！",
        alert_err_merged_empty: "“merged/”文件夹为空！应用前请先在“诊断与合并”选项卡中合并模组。",
        alert_err_explorer: "无法在资源管理器中打开文件夹。",
        alert_err_organize: "整理文件夹失败。",
        alert_err_create_mod: "创建模组文件夹失败。",
        alert_err_scan: "诊断错误: {msg}",
        alert_err_deploy: "部署错误: {msg}",
        alert_err_restore: "恢复错误: {msg}",
        alert_err_launch: "启动错误: {msg}",
        alert_err_generic: "发生错误: {msg}",
        filter_no_files: "在此筛选条件下未找到任何文件。",
        launcher_status_active_session: "当前会话",
        btn_merging: "正在合并...",
        details_discrepancies: "详情与内部冲突（{count}项）：",
        present_in: "包含于",
        launch_mode_label: "启动方式:",
        launch_mode_auto: "自动检测 (Steam / Exe)",
        launch_mode_steam: "Steam (steam://run/570940)",
        launch_mode_direct: "直接执行 (.exe)",
        alert_launch_steam_success: "正在通过 Steam 启动 Dark Souls Remastered...",
        alert_launch_direct_success: "正在直接通过可执行文件启动 Dark Souls Remastered...",
        detected_steam_badge: "已自动选择 Steam",
        detected_direct_badge: "已自动选择非Steam / 独立版"
    }
};

/* ============================================================
   GLOBAL APPLICATION STATE
   ============================================================ */
let currentLang = 'en';
let currentModFolders = [];
let currentScanData = null;
let currentFilter = 'all';

let disabledFilesState = {};
try {
    const saved = localStorage.getItem('soulsconflict_disabled_files');
    if (saved) disabledFilesState = JSON.parse(saved);
} catch (e) {
    console.error('Failed to load disabled files state:', e);
    disabledFilesState = {};
}

function saveDisabledFilesState() {
    try {
        localStorage.setItem('soulsconflict_disabled_files', JSON.stringify(disabledFilesState));
    } catch (e) {
        console.error('Failed to save disabled files state:', e);
    }
}

function resetAllDisabledFiles() {
    disabledFilesState = {};
    saveDisabledFilesState();
    runScan();
}

function toggleFileInMod(modName, relPath, e) {
    if (e) e.stopPropagation();
    if (!disabledFilesState[modName]) {
        disabledFilesState[modName] = [];
    }
    const idx = disabledFilesState[modName].indexOf(relPath);
    if (idx >= 0) {
        disabledFilesState[modName].splice(idx, 1);
        if (disabledFilesState[modName].length === 0) {
            delete disabledFilesState[modName];
        }
    } else {
        disabledFilesState[modName].push(relPath);
    }
    saveDisabledFilesState();
    runScan();
}

/* ============================================================
   DISABLED MODS (CHECKBOX TOGGLE) STATE
   ============================================================ */
let disabledModIds = new Set();
try {
    const savedMods = localStorage.getItem('soulsconflict_disabled_mods');
    if (savedMods) {
        const parsed = JSON.parse(savedMods);
        if (Array.isArray(parsed)) disabledModIds = new Set(parsed);
    }
} catch (e) {
    console.error('Failed to load disabled mods state:', e);
    disabledModIds = new Set();
}

function saveDisabledModIds() {
    try {
        localStorage.setItem('soulsconflict_disabled_mods', JSON.stringify(Array.from(disabledModIds)));
    } catch (e) {
        console.error('Failed to save disabled mods state:', e);
    }
}

function isModEnabled(modId) {
    return !disabledModIds.has(modId);
}

function toggleModEnabled(modId, e) {
    if (e) e.stopPropagation();
    if (disabledModIds.has(modId)) {
        disabledModIds.delete(modId);
    } else {
        disabledModIds.add(modId);
    }
    saveDisabledModIds();
    renderModSlots();
    loadDeployerStatus();
}

/* ============================================================
   DSRR (DARK SOULS RE-REMASTERED) VISUAL LAYER PRESET
   ============================================================ */
function findDsrrMod(onlyEnabled = true) {
    if (!currentModFolders) return null;
    return currentModFolders.find(m => {
        if (onlyEnabled && !isModEnabled(m.id)) return false;
        const name = (m.name || '').toLowerCase();
        const id = (m.id || '').toLowerCase();
        const path = (m.relative_path || '').toLowerCase();
        return name.includes('re-remastered') || name.includes('dsrr') ||
               id.includes('re-remastered') || id.includes('dsrr') ||
               path.includes('re-remastered') || path.includes('dsrr');
    });
}

let dsrrPresetMode = 'strict';
try {
    const savedPresetMode = localStorage.getItem('soulsconflict_dsrr_preset_mode');
    if (savedPresetMode) dsrrPresetMode = savedPresetMode;
} catch (e) {}

function isDsrrMapStructureFile(relPath) {
    const lower = relPath.toLowerCase().replace(/\\/g, '/');
    if (lower.startsWith('map/')) {
        const isTex = lower.endsWith('.tpfbdt') || lower.endsWith('.tpfbhd')
            || lower.endsWith('.tpf.dcx') || lower.endsWith('.tpf');
        return !isTex;
    }
    return false;
}

function isDsrrLogicFile(relPath) {
    const lower = relPath.toLowerCase().replace(/\\/g, '/');
    if (lower.includes('gameparam.parambnd')) return true;
    if (lower.startsWith('event/')) return true;
    if (lower.startsWith('script/')) return true;
    if (lower.endsWith('.esd.dcx')) return true;
    if (lower.endsWith('.anibnd.dcx')) return true;
    if (lower.endsWith('.chresdbnd.dcx')) return true;
    if (isDsrrMapStructureFile(relPath)) return true;
    return false;
}

function isDsrrStrictNonAssetFile(relPath) {
    const lower = relPath.toLowerCase().replace(/\\/g, '/');
    if (lower.startsWith('event/')) return true;
    if (lower.startsWith('script/')) return true;
    if (lower.endsWith('.msb')) return true;
    if (lower.endsWith('.anibnd.dcx')) return true;
    if (lower.includes('.esd.') || lower.endsWith('.esd.dcx') || lower.endsWith('.chresdbnd.dcx')) return true;
    if (lower.startsWith('param/')) return true;
    if (lower.startsWith('menu/')) return true;
    if (lower.startsWith('sfx/')) return true;
    if (isDsrrMapStructureFile(relPath)) return true;
    return false;
}

function updateDsrrPresetBanner() {
    const container = document.getElementById('dsrrPresetBannerContainer');
    if (!container) return;

    const dsrrMod = findDsrrMod(true);
    const activeMods = currentModFolders ? currentModFolders.filter(m => isModEnabled(m.id)) : [];
    if (!dsrrMod || !currentScanData || activeMods.length < 2) {
        container.classList.add('hidden');
        container.innerHTML = '';
        return;
    }

    const disabledList = disabledFilesState[dsrrMod.name] || [];
    const isVisualLayerActive = disabledList.length > 0;

    // Auto-update preset if already active in strict mode but missing map structure exclusions
    if (isVisualLayerActive && dsrrPresetMode === 'strict' && disabledList.length < 2000) {
        setTimeout(() => applyDsrrVisualPreset('strict'), 100);
    }

    container.classList.remove('hidden');

    if (isVisualLayerActive) {
        const isStrict = dsrrPresetMode === 'strict';
        const tagText = isStrict ? t('dsrr_preset_active_tag_strict') : t('dsrr_preset_active_tag');
        const titleText = isStrict
            ? t('dsrr_preset_active_title_strict', { count: disabledList.length })
            : t('dsrr_preset_active_title', { count: disabledList.length });
        const descText = isStrict ? t('dsrr_preset_active_desc_strict') : t('dsrr_preset_active_desc');

        const switchBtnHtml = isStrict
            ? `<button type="button" class="btn btn-secondary btn-sm" onclick="applyDsrrVisualPreset('standard')">${t('dsrr_preset_btn_enable')}</button>`
            : `<button type="button" class="btn btn-gold btn-sm" onclick="applyDsrrVisualPreset('strict')">${t('dsrr_preset_btn_enable_strict')}</button>`;

        container.innerHTML = `
            <div class="dsrr-preset-banner dsrr-preset-active">
                <div class="dsrr-preset-left">
                    <div class="dsrr-preset-title-row">
                        <span class="dsrr-badge-tag active">${tagText}</span>
                        <span class="dsrr-preset-title">${titleText}</span>
                    </div>
                    <div class="dsrr-preset-desc">
                        ${descText}
                    </div>
                </div>
                <div class="dsrr-preset-right" style="display:flex; gap:8px; align-items:center;">
                    ${switchBtnHtml}
                    <button type="button" class="btn btn-secondary btn-sm" onclick="applyDsrrVisualPreset('disable')">
                        ${t('dsrr_preset_btn_disable')}
                    </button>
                </div>
            </div>
        `;
    } else {
        container.innerHTML = `
            <div class="dsrr-preset-banner">
                <div class="dsrr-preset-left">
                    <div class="dsrr-preset-title-row">
                        <span class="dsrr-badge-tag">${t('dsrr_preset_tag_detected')}</span>
                        <span class="dsrr-preset-title">${t('dsrr_preset_title')}</span>
                    </div>
                    <div class="dsrr-preset-desc">
                        ${t('dsrr_preset_desc')}
                    </div>
                </div>
                <div class="dsrr-preset-right" style="display:flex; gap:8px; align-items:center;">
                    <button type="button" class="btn btn-secondary btn-sm" onclick="applyDsrrVisualPreset('standard')">
                        ${t('dsrr_preset_btn_enable')}
                    </button>
                    <button type="button" class="btn btn-gold btn-sm" onclick="applyDsrrVisualPreset('strict')">
                        ${t('dsrr_preset_btn_enable_strict')}
                    </button>
                </div>
            </div>
        `;
    }
}

async function applyDsrrVisualPreset(action) {
    const dsrrMod = findDsrrMod();
    if (!dsrrMod) return;

    if (action === 'standard' || action === 'strict' || action === 'enable') {
        const mode = (action === 'enable') ? 'strict' : action;
        dsrrPresetMode = mode;
        try {
            localStorage.setItem('soulsconflict_dsrr_preset_mode', mode);
        } catch (e) {}

        if (!currentScanData || !currentScanData.files) {
            await runScan();
        }
        if (!currentScanData || !currentScanData.files) return;

        let targetFiles = [];
        if (mode === 'strict') {
            targetFiles = currentScanData.files
                .filter(f => f.present_in_mods && f.present_in_mods.includes(dsrrMod.name))
                .filter(f => f.present_in_mods.length > 1 || isDsrrStrictNonAssetFile(f.relative_path))
                .map(f => f.relative_path);
        } else {
            targetFiles = currentScanData.files
                .filter(f => f.present_in_mods && f.present_in_mods.includes(dsrrMod.name))
                .filter(f => (f.present_in_mods.length > 1 && isDsrrLogicFile(f.relative_path)) || isDsrrMapStructureFile(f.relative_path))
                .map(f => f.relative_path);
        }

        if (targetFiles.length > 0) {
            disabledFilesState[dsrrMod.name] = targetFiles;
            saveDisabledFilesState();
            await runScan();
        }
    } else {
        delete disabledFilesState[dsrrMod.name];
        try {
            localStorage.removeItem('soulsconflict_dsrr_preset_mode');
        } catch (e) {}
        saveDisabledFilesState();
        await runScan();
    }
}

/* ============================================================
   i18n HELPER FUNCTIONS
   ============================================================ */
function getInitialLanguage() {
    const saved = localStorage.getItem('soulsconflict_lang');
    if (saved && TRANSLATIONS[saved]) return saved;

    const nav = (navigator.language || navigator.userLanguage || '').toLowerCase();
    if (nav.startsWith('pt')) return 'pt';
    if (nav.startsWith('it')) return 'it';
    if (nav.startsWith('fr')) return 'fr';
    if (nav.startsWith('ja')) return 'ja';
    if (nav.startsWith('zh')) return 'zh';

    return 'en'; // Default primary language is English
}

function t(key, params = {}) {
    const dict = TRANSLATIONS[currentLang] || TRANSLATIONS['en'];
    let str = dict[key] !== undefined ? dict[key] : (TRANSLATIONS['en'][key] !== undefined ? TRANSLATIONS['en'][key] : key);
    for (const [k, v] of Object.entries(params)) {
        str = str.replace(new RegExp(`\\{${k}\\}`, 'g'), v);
    }
    return str;
}

function setLanguage(lang) {
    if (!TRANSLATIONS[lang]) lang = 'en';
    currentLang = lang;
    localStorage.setItem('soulsconflict_lang', lang);
    document.documentElement.lang = lang;

    const select = document.getElementById('langSelect');
    if (select && select.value !== lang) {
        select.value = lang;
    }

    // Translate all static data-i18n elements
    document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        el.innerHTML = t(key);
    });

    document.querySelectorAll('[data-i18n-title]').forEach(el => {
        const key = el.getAttribute('data-i18n-title');
        el.setAttribute('title', t(key));
    });

    document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
        const key = el.getAttribute('data-i18n-placeholder');
        el.setAttribute('placeholder', t(key));
    });

    // Re-render dynamic elements
    renderModSlots();
    if (currentScanData) {
        displayResults(currentScanData);
    }
    loadDeployerStatus();
}

/* ============================================================
   CUSTOM THEMED MODAL CONTROLLER (REPLACES NATIVE ALERT/CONFIRM/PROMPT)
   ============================================================ */
let modalResolve = null;

function showCustomAlert(message, title) {
    return new Promise((resolve) => {
        const overlay = document.getElementById('customModalOverlay');
        const titleEl = document.getElementById('customModalTitle');
        const msgEl = document.getElementById('customModalMessage');
        const inputEl = document.getElementById('customModalInput');
        const okBtn = document.getElementById('customModalOkBtn');
        const cancelBtn = document.getElementById('customModalCancelBtn');
        const iconEl = document.getElementById('customModalIcon');

        titleEl.textContent = title || t('app_title');
        msgEl.textContent = message;
        inputEl.classList.add('hidden');
        cancelBtn.classList.add('hidden');
        okBtn.textContent = t('modal_ok');
        iconEl.innerHTML = '<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>';

        overlay.classList.remove('hidden');
        okBtn.focus();

        modalResolve = (val) => {
            overlay.classList.add('hidden');
            resolve(val);
        };

        okBtn.onclick = () => modalResolve(true);
    });
}

function showCustomConfirm(message, title) {
    return new Promise((resolve) => {
        const overlay = document.getElementById('customModalOverlay');
        const titleEl = document.getElementById('customModalTitle');
        const msgEl = document.getElementById('customModalMessage');
        const inputEl = document.getElementById('customModalInput');
        const okBtn = document.getElementById('customModalOkBtn');
        const cancelBtn = document.getElementById('customModalCancelBtn');
        const iconEl = document.getElementById('customModalIcon');

        titleEl.textContent = title || t('modal_title_confirm');
        msgEl.textContent = message;
        inputEl.classList.add('hidden');
        cancelBtn.classList.remove('hidden');
        cancelBtn.textContent = t('modal_cancel');
        okBtn.textContent = t('modal_confirm');
        iconEl.innerHTML = '<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>';

        overlay.classList.remove('hidden');
        okBtn.focus();

        modalResolve = (val) => {
            overlay.classList.add('hidden');
            resolve(val);
        };

        okBtn.onclick = () => modalResolve(true);
        cancelBtn.onclick = () => modalResolve(false);
    });
}

function showCustomPrompt(message, defaultValue = '', title) {
    return new Promise((resolve) => {
        const overlay = document.getElementById('customModalOverlay');
        const titleEl = document.getElementById('customModalTitle');
        const msgEl = document.getElementById('customModalMessage');
        const inputEl = document.getElementById('customModalInput');
        const okBtn = document.getElementById('customModalOkBtn');
        const cancelBtn = document.getElementById('customModalCancelBtn');
        const iconEl = document.getElementById('customModalIcon');

        titleEl.textContent = title || t('modal_title_prompt');
        msgEl.textContent = message;
        inputEl.value = defaultValue;
        inputEl.classList.remove('hidden');
        cancelBtn.classList.remove('hidden');
        cancelBtn.textContent = t('modal_cancel');
        okBtn.textContent = t('modal_ok');
        iconEl.innerHTML = '<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 20h9"/><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/></svg>';

        overlay.classList.remove('hidden');
        inputEl.focus();
        inputEl.select();

        modalResolve = (val) => {
            overlay.classList.add('hidden');
            resolve(val);
        };

        okBtn.onclick = () => modalResolve(inputEl.value);
        cancelBtn.onclick = () => modalResolve(null);
        inputEl.onkeydown = (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                modalResolve(inputEl.value);
            } else if (e.key === 'Escape') {
                e.preventDefault();
                modalResolve(null);
            }
        };
    });
}

window.addEventListener('keydown', (e) => {
    const overlay = document.getElementById('customModalOverlay');
    if (!overlay || overlay.classList.contains('hidden')) return;
    if (e.key === 'Escape' && modalResolve) {
        modalResolve(null);
    }
});

/* ============================================================
   INITIALIZATION & EVENT LISTENERS
   ============================================================ */
document.addEventListener('DOMContentLoaded', () => {
    // 1. Initialize Language System
    const langSelect = document.getElementById('langSelect');
    const initialLang = getInitialLanguage();
    if (langSelect) {
        langSelect.value = initialLang;
        langSelect.addEventListener('change', (e) => setLanguage(e.target.value));
    }
    setLanguage(initialLang);

    // 2. Buttons and Navigation
    const btnScan = document.getElementById('btnScan');
    const btnMerge = document.getElementById('btnMerge');
    const tabBtns = document.querySelectorAll('.tab-btn');
    const btnRefreshStatus = document.getElementById('btnRefreshStatus');
    const btnOpenRootFolder = document.getElementById('btnOpenRootFolder');
    const btnOpenMerged = document.getElementById('btnOpenMerged');
    const btnAddModSlot = document.getElementById('btnAddModSlot');

    if (btnScan) btnScan.addEventListener('click', runScan);
    if (btnMerge) btnMerge.addEventListener('click', runMerge);
    if (btnRefreshStatus) btnRefreshStatus.addEventListener('click', loadStatus);
    if (btnOpenRootFolder) btnOpenRootFolder.addEventListener('click', () => openFolder('mods'));
    if (btnOpenMerged) btnOpenMerged.addEventListener('click', () => openFolder('merged'));
    if (btnAddModSlot) btnAddModSlot.addEventListener('click', addNewModFolder);

    const modeRadios = document.querySelectorAll('input[name="resolutionMode"]');
    modeRadios.forEach(radio => {
        radio.addEventListener('change', () => {
            document.querySelectorAll('.mode-option-card').forEach(c => c.classList.remove('active'));
            radio.closest('.mode-option-card').classList.add('active');
        });
    });

    tabBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            tabBtns.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            currentFilter = btn.dataset.filter;
            renderFileList();
        });
    });

    const btnImportModArchive = document.getElementById('btnImportModArchive');
    const inputModArchive = document.getElementById('inputModArchive');
    if (btnImportModArchive && inputModArchive) {
        btnImportModArchive.addEventListener('click', () => {
            inputModArchive.click();
        });
        inputModArchive.addEventListener('change', async (e) => {
            if (e.target.files && e.target.files.length > 0) {
                await uploadModArchives(e.target.files);
                inputModArchive.value = '';
            }
        });
    }

    // Drag-and-drop archive support (.zip, .rar, .7z) on config-card
    const configCard = document.querySelector('.config-card');
    if (configCard) {
        ['dragenter', 'dragover'].forEach(eventName => {
            configCard.addEventListener(eventName, (e) => {
                e.preventDefault();
                e.stopPropagation();
                configCard.classList.add('dragover-active');
            }, false);
        });

        ['dragleave', 'drop'].forEach(eventName => {
            configCard.addEventListener(eventName, (e) => {
                e.preventDefault();
                e.stopPropagation();
                configCard.classList.remove('dragover-active');
            }, false);
        });

        configCard.addEventListener('drop', async (e) => {
            const dt = e.dataTransfer;
            const files = dt && dt.files;
            if (files && files.length > 0) {
                const archiveFiles = Array.from(files).filter(f => {
                    const ext = f.name.split('.').pop().toLowerCase();
                    return ext === 'zip' || ext === 'rar' || ext === '7z';
                });
                if (archiveFiles.length > 0) {
                    await uploadModArchives(archiveFiles);
                }
            }
        });
    }

    // Main Navigation Tabs
    const navTabMerge = document.getElementById('navTabMerge');
    const navTabLauncher = document.getElementById('navTabLauncher');
    if (navTabMerge) navTabMerge.addEventListener('click', () => switchMainTab('merge'));
    if (navTabLauncher) navTabLauncher.addEventListener('click', () => switchMainTab('launcher'));

    // Launcher Actions
    const btnDeployMods = document.getElementById('btnDeployMods');
    const btnRestoreVanilla = document.getElementById('btnRestoreVanilla');
    const btnLaunchGame = document.getElementById('btnLaunchGame');
    const btnLaunchSeamless = document.getElementById('btnLaunchSeamless');
    const btnSaveGamePath = document.getElementById('btnSaveGamePath');
    const btnOpenGameFolder = document.getElementById('btnOpenGameFolder');
    const btnRefreshDeployer = document.getElementById('btnRefreshDeployer');

    if (btnDeployMods) btnDeployMods.addEventListener('click', deployMods);
    if (btnRestoreVanilla) btnRestoreVanilla.addEventListener('click', restoreVanilla);
    if (btnLaunchGame) btnLaunchGame.addEventListener('click', () => launchGame(null));
    if (btnLaunchSeamless) btnLaunchSeamless.addEventListener('click', () => launchGame('seamless'));
    if (btnSaveGamePath) btnSaveGamePath.addEventListener('click', saveGamePath);
    if (btnOpenGameFolder) btnOpenGameFolder.addEventListener('click', openGameFolder);
    if (btnRefreshDeployer) btnRefreshDeployer.addEventListener('click', loadDeployerStatus);

    // Auto-load status on launch
    loadStatus();
    loadDeployerStatus();

    let lastFocusRefresh = Date.now();
    window.addEventListener('focus', () => {
        const now = Date.now();
        if (now - lastFocusRefresh > 3000) {
            lastFocusRefresh = now;
            loadStatus();
            loadDeployerStatus();
        }
    });
});

function switchMainTab(tab) {
    const navTabMerge = document.getElementById('navTabMerge');
    const navTabLauncher = document.getElementById('navTabLauncher');
    const viewMergeTab = document.getElementById('viewMergeTab');
    const viewLauncherTab = document.getElementById('viewLauncherTab');

    if (tab === 'merge') {
        navTabMerge.classList.add('active');
        navTabLauncher.classList.remove('active');
        viewMergeTab.classList.remove('hidden');
        viewLauncherTab.classList.add('hidden');
    } else {
        navTabLauncher.classList.add('active');
        navTabMerge.classList.remove('active');
        viewLauncherTab.classList.remove('hidden');
        viewMergeTab.classList.add('hidden');
        loadDeployerStatus();
    }
}

async function loadStatus() {
    try {
        const response = await fetch('/api/status');
        if (!response.ok) return;
        const data = await response.json();

        // Preserve current user-defined order if already loaded
        if (currentModFolders.length > 0) {
            const mapById = new Map();
            data.mods.forEach(m => mapById.set(m.id, m));

            const updated = [];
            currentModFolders.forEach(m => {
                if (mapById.has(m.id)) {
                    const fresh = mapById.get(m.id);
                    if (m.selected_variant) {
                        fresh.selected_variant = m.selected_variant;
                    }
                    updated.push(fresh);
                    mapById.delete(m.id);
                }
            });
            mapById.forEach(m => updated.push(m));
            currentModFolders = updated;
        } else {
            currentModFolders = data.mods;
        }

        renderModSlots();

        // Update merged path
        const elMergedPath = document.getElementById('fullPathMerged');
        if (elMergedPath && data.merged) {
            elMergedPath.innerText = data.merged.full_path;
        }
    } catch (e) {
        console.warn('Could not load status:', e);
    }
}

function renderModSlots() {
    const grid = document.getElementById('modSlotsGrid');
    if (!grid) return;
    grid.innerHTML = '';

    if (currentModFolders.length === 0) {
        grid.innerHTML = `
            <div class="empty-mods-state">
                <div class="empty-mods-icon">
                    <svg width="38" height="38" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" style="color: var(--gold); opacity: 0.9;"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>
                </div>
                <div class="empty-mods-title">${t('empty_mods_title')}</div>
                <p class="empty-mods-text">${t('empty_mods_desc')}</p>
                <div class="empty-mods-actions">
                    <button class="btn btn-primary" onclick="openFolder('mods')">
                        ${t('open_mods_folder')}
                    </button>
                    <button class="btn btn-secondary" onclick="addNewModFolder()">
                        ${t('btn_create_mod_folder')}
                    </button>
                    <button class="btn btn-secondary" onclick="loadStatus()">
                        ${t('btn_refresh')}
                    </button>
                </div>
            </div>
        `;
        return;
    }

    let activeOrder = 0;
    currentModFolders.forEach((mod, index) => {
        const isEnabled = isModEnabled(mod.id);
        if (isEnabled) activeOrder++;

        const row = document.createElement('div');
        row.className = `mod-row-item ${isEnabled ? '' : 'mod-row-disabled'}`;

        const isTop = index === 0;
        const isBottom = index === currentModFolders.length - 1;

        const priorityLabel = isEnabled ? (activeOrder === 1 ? '#1' : `#${activeOrder}`) : '-';
        const pillClass = isEnabled
            ? (activeOrder === 1 ? 'priority-pill priority-first' : 'priority-pill')
            : 'priority-pill priority-disabled';

        const isReady = mod.file_count > 0;
        const badgeClass = isEnabled
            ? (isReady ? 'badge-ready' : 'badge-empty')
            : 'badge-mod-disabled';
        const badgeText = isEnabled
            ? (isReady ? t('badge_ready', { count: mod.file_count }) : t('badge_empty'))
            : t('badge_mod_disabled');

        let subfolderInfo = '';
        if (mod.subfolder_detected && mod.subfolder_detected !== mod.name) {
            subfolderInfo = `<span class="mod-row-subfolder">${escapeHtml(mod.subfolder_detected)}</span>`;
        }

        let variantSelectHtml = '';
        if (mod.variants && mod.variants.length > 1) {
            const activeVar = mod.selected_variant || mod.variants[0];
            const opts = mod.variants.map(v => `<option value="${escapeHtml(v)}" ${v === activeVar ? 'selected' : ''}>${escapeHtml(v)}</option>`).join('');
            variantSelectHtml = `
                <div class="mod-variant-select-box" title="${t('variant_title')}">
                    <span class="variant-label">${t('variant_label')}</span>
                    <select class="variant-dropdown" onchange="onSelectVariant('${escapeHtml(mod.id)}', this.value)">
                        ${opts}
                    </select>
                </div>
            `;
        }

        let structureWarnHtml = '';
        if (mod.has_structure_issues) {
            structureWarnHtml = `<span class="badge-structure-warn" title="${t('badge_auto_mapped_title')}">${t('badge_auto_mapped')}</span>`;
        }

        let dsrrPresetTagHtml = '';
        const dsrrMod = findDsrrMod(false);
        if (dsrrMod && dsrrMod.id === mod.id) {
            const disabledList = disabledFilesState[mod.name] || [];
            const isVisualLayerActive = disabledList.length > 0;
            const isStrict = isVisualLayerActive && dsrrPresetMode === 'strict';
            const tagLabel = isVisualLayerActive
                ? (isStrict ? `${t('mod_tag_visual_layer_strict')} (ON)` : `${t('mod_tag_visual_layer')} (ON)`)
                : t('mod_tag_visual_layer');
            const nextMode = isVisualLayerActive ? (isStrict ? 'disable' : 'strict') : 'strict';
            dsrrPresetTagHtml = `
                <span class="btn-visual-layer-tag ${isVisualLayerActive ? 'active' : ''} ${isStrict ? 'strict' : ''}" 
                      onclick="applyDsrrVisualPreset('${nextMode}')" 
                      title="${isStrict ? t('dsrr_preset_active_desc_strict') : t('dsrr_preset_desc')}">
                    ${tagLabel}
                </span>
            `;
        }

        let fixModBtnHtml = '';
        if (mod.has_structure_issues) {
            fixModBtnHtml = `
                <button class="btn btn-fix-mod btn-sm" onclick="fixModStructure('${escapeHtml(mod.id)}')" title="${t('btn_organize_title')}">
                    ${t('btn_organize')}
                </button>
            `;
        }

        row.innerHTML = `
            <div class="mod-row-left">
                <div class="mod-row-priority">
                    <label class="mod-checkbox-label" title="${isEnabled ? t('mod_disable_tooltip') : t('mod_enable_tooltip')}">
                        <input type="checkbox" class="mod-enable-checkbox" ${isEnabled ? 'checked' : ''} onchange="toggleModEnabled('${escapeHtml(mod.id)}', event)">
                    </label>
                    <span class="${pillClass}">${priorityLabel}</span>
                    <div class="slot-order-btns">
                        <button class="btn-arrow" ${isTop ? 'disabled' : ''} onclick="moveMod(${index}, -1)" title="${t('move_up_title')}">▲</button>
                        <button class="btn-arrow" ${isBottom ? 'disabled' : ''} onclick="moveMod(${index}, 1)" title="${t('move_down_title')}">▼</button>
                    </div>
                </div>

                <div class="mod-row-info">
                    <div class="mod-row-title-line">
                        <span class="mod-row-title" title="${escapeHtml(mod.name)}">${escapeHtml(mod.name)}</span>
                        <span class="mod-row-tag">${escapeHtml(mod.relative_path)}</span>
                        ${variantSelectHtml}
                        ${structureWarnHtml}
                        ${dsrrPresetTagHtml}
                    </div>
                    ${subfolderInfo}
                </div>
            </div>

            <div class="mod-row-right">
                <span class="slot-status-badge ${badgeClass}">${badgeText}</span>
                ${fixModBtnHtml}
                <button class="btn btn-secondary btn-sm" onclick="openFolder('${escapeHtml(mod.id)}')">
                    ${t('btn_open')}
                </button>
                <button class="btn-delete-mod" onclick="deleteModFolder('${escapeHtml(mod.id)}')" title="${t('btn_delete_title')}">
                    ✕
                </button>
            </div>
        `;

        grid.appendChild(row);
    });
}

function moveMod(index, direction) {
    const target = index + direction;
    if (target < 0 || target >= currentModFolders.length) return;
    const temp = currentModFolders[index];
    currentModFolders[index] = currentModFolders[target];
    currentModFolders[target] = temp;
    renderModSlots();
}

async function addNewModFolder() {
    const customName = await showCustomPrompt(t('prompt_new_folder'), '', t('modal_title_prompt'));
    if (customName === null) return;

    try {
        const response = await fetch('/api/create_mod_folder', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name: customName.trim() })
        });

        if (!response.ok) {
            throw new Error(t('alert_err_create_mod'));
        }

        await loadStatus();
    } catch (err) {
        await showCustomAlert(t('alert_err_generic', { msg: err.message }), t('modal_title_error'));
    }
}

async function deleteModFolder(id) {
    const confirmed = await showCustomConfirm(t('confirm_delete_folder', { id }), t('modal_title_confirm'));
    if (!confirmed) {
        return;
    }
    try {
        const res = await fetch('/api/delete_mod_folder', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ target: id })
        });
        if (res.ok) {
            disabledModIds.delete(id);
            saveDisabledModIds();
            await loadStatus();
        }
    } catch (e) {
        await showCustomAlert(t('alert_err_generic', { msg: e.message }), t('modal_title_error'));
    }
}

async function uploadModArchives(files) {
    if (!files || files.length === 0) return;

    const fileList = Array.from(files);
    let importedCount = 0;

    for (let i = 0; i < fileList.length; i++) {
        const file = fileList[i];
        const ext = file.name.split('.').pop().toLowerCase();
        if (ext !== 'zip' && ext !== 'rar' && ext !== '7z') {
            await showCustomAlert(t('alert_err_not_archive') || `File '${file.name}' is not a supported mod archive (.zip, .rar, .7z).`, t('modal_title_error'));
            continue;
        }

        const loadingBox = document.getElementById('loadingBox');
        const loadingTitle = loadingBox ? loadingBox.querySelector('h3') : null;
        const loadingDesc = loadingBox ? loadingBox.querySelector('p') : null;
        const prevTitle = loadingTitle ? loadingTitle.textContent : '';
        const prevDesc = loadingDesc ? loadingDesc.textContent : '';

        if (loadingBox) {
            if (loadingTitle) loadingTitle.textContent = (t('uploading_archive_progress') || "Extracting & importing mod '{name}'...").replace('{name}', file.name);
            if (loadingDesc) loadingDesc.textContent = "Unpacking files natively and organizing folder structure...";
            loadingBox.classList.remove('hidden');
        }

        try {
            const url = `/api/import_mod_archive?name=${encodeURIComponent(file.name)}`;
            const res = await fetch(url, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/octet-stream',
                    'X-Filename': encodeURIComponent(file.name)
                },
                body: file
            });
            const data = await res.json();
            if (res.ok && data.success) {
                importedCount++;
                await showCustomAlert(data.message || `Mod '${data.mod_name}' imported successfully!`, t('modal_title_success'));
            } else {
                await showCustomAlert(data.message || `Failed to import mod from '${file.name}'.`, t('modal_title_error'));
            }
        } catch (e) {
            await showCustomAlert(t('alert_err_generic', { msg: e.message }), t('modal_title_error'));
        } finally {
            if (loadingBox) {
                if (loadingTitle) loadingTitle.textContent = prevTitle;
                if (loadingDesc) loadingDesc.textContent = prevDesc;
                loadingBox.classList.add('hidden');
            }
        }
    }

    if (importedCount > 0) {
        await loadStatus();
    }
}

async function openFolder(target) {
    try {
        const res = await fetch('/api/open_folder', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ target })
        });
        if (!res.ok) {
            await showCustomAlert(t('alert_err_explorer'), t('modal_title_error'));
        }
    } catch (err) {
        await showCustomAlert(t('alert_err_generic', { msg: err.message }), t('modal_title_error'));
    }
}

function onSelectVariant(modId, val) {
    const m = currentModFolders.find(x => x.id === modId);
    if (m) {
        m.selected_variant = val;
    }
}

async function fixModStructure(modId) {
    const confirmed = await showCustomConfirm(t('confirm_fix_structure', { id: modId }), t('modal_title_confirm'));
    if (!confirmed) {
        return;
    }
    try {
        const res = await fetch('/api/fix_mod_structure', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ target: modId })
        });
        const data = await res.json();
        if (data.success) {
            await showCustomAlert(
                t('alert_fix_structure_success', { count: data.organized_count }),
                t('modal_title_success')
            );
            await loadStatus();
        } else {
            await showCustomAlert(data.message || t('alert_err_organize'), t('modal_title_error'));
        }
    } catch (e) {
        await showCustomAlert(t('alert_err_generic', { msg: e.message }), t('modal_title_error'));
    }
}

async function runScan() {
    const activeMods = currentModFolders.filter(m => isModEnabled(m.id));
    if (activeMods.length < 2) {
        await showCustomAlert(t('alert_min_mods_active'), t('modal_title_info'));
        return;
    }

    const modsToScan = activeMods.map(m => ({
        name: m.name,
        path: m.relative_path,
        variant: m.selected_variant || null,
        disabled_files: disabledFilesState[m.name] || []
    }));

    const loadingBox = document.getElementById('loadingBox');
    const resultsSection = document.getElementById('resultsSection');

    loadingBox.classList.remove('hidden');
    resultsSection.classList.add('hidden');

    try {
        const response = await fetch('/api/scan', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ mods: modsToScan })
        });

        if (!response.ok) {
            const err = await response.text();
            throw new Error(err || 'Failed to execute scan.');
        }

        const data = await response.json();
        currentScanData = data;
        displayResults(data);
    } catch (err) {
        await showCustomAlert(t('alert_err_scan', { msg: err.message }), t('modal_title_error'));
    } finally {
        loadingBox.classList.add('hidden');
    }
}

function displayResults(data) {
    const resultsSection = document.getElementById('resultsSection');
    resultsSection.classList.remove('hidden');

    const s = data.summary;
    document.getElementById('statTotal').innerText = s.total_examined;
    document.getElementById('statSafe').innerText = s.safe_count;
    document.getElementById('statMergeable').innerText = s.mergeable_count;
    document.getElementById('statConflict').innerText = s.conflict_count;

    document.getElementById('countTabAll').innerText = s.total_examined;
    document.getElementById('countTabSafe').innerText = s.safe_count;
    document.getElementById('countTabMergeable').innerText = s.mergeable_count;
    document.getElementById('countTabConflict').innerText = s.conflict_count;

    const mergeMsg = document.getElementById('mergeMessage');

    if (s.conflict_count === 0) {
        mergeMsg.innerHTML = t('all_compatible_msg', { total: s.total_mods });
    } else {
        mergeMsg.innerHTML = t('conflicts_found_msg', { count: s.conflict_count });
    }

    renderFileList();
    updateDsrrPresetBanner();
    resultsSection.scrollIntoView({ behavior: 'smooth' });
}

function renderFileList() {
    if (!currentScanData) return;
    updateDsrrPresetBanner();

    const list = document.getElementById('fileList');
    list.innerHTML = '';

    const filtered = currentScanData.files.filter(f => {
        if (currentFilter === 'all') return true;
        if (currentFilter === 'safe') return f.level === 'Safe';
        if (currentFilter === 'mergeable') return f.level === 'Mergeable';
        if (currentFilter === 'conflict') return f.level === 'Conflict';
        return true;
    });

    if (filtered.length === 0) {
        list.innerHTML = `<div style="text-align:center; color:#7d838a; padding: 24px;">${t('filter_no_files')}</div>`;
        return;
    }

    const totalDisabledCount = Object.values(disabledFilesState).reduce((acc, arr) => acc + (arr ? arr.length : 0), 0);
    if (totalDisabledCount > 0) {
        const banner = document.createElement('div');
        banner.style.cssText = "margin-bottom:14px; padding:10px 14px; background:rgba(74,100,128,0.12); border:1px solid rgba(74,100,128,0.3); border-radius:6px; display:flex; justify-content:space-between; align-items:center; font-size:0.85rem; color:#9bb7cf;";
        banner.innerHTML = `
            <div>
                ${t('custom_toggles_banner', { count: totalDisabledCount })}
            </div>
            <button type="button" style="cursor:pointer; background:rgba(74,100,128,0.2); border:1px solid rgba(74,100,128,0.4); color:#b8cde0; padding:3px 10px; border-radius:4px; font-size:0.78rem;" onclick="resetAllDisabledFiles()">
                ${t('btn_restore_all_toggles')}
            </button>
        `;
        list.appendChild(banner);
    }

    filtered.forEach(file => {
        const item = document.createElement('div');
        item.className = 'file-item';

        const isMultiMod = file.present_in_mods && file.present_in_mods.length > 1;
        const fileDisabledMods = isMultiMod
            ? file.present_in_mods.filter(m => (disabledFilesState[m] || []).includes(file.relative_path))
            : [];
        const fileActiveMods = isMultiMod
            ? file.present_in_mods.filter(m => !(disabledFilesState[m] || []).includes(file.relative_path))
            : (file.present_in_mods || []);

        const isResolvedByToggle = isMultiMod && fileDisabledMods.length > 0 && fileActiveMods.length === 1;

        // Current real-time effective winner:
        const currentWinner = fileActiveMods.length > 0 ? fileActiveMods[0] : (isMultiMod ? null : (file.present_in_mods ? file.present_in_mods[0] : null));
        const otherActiveMods = fileActiveMods.slice(1);
        const disabledMods = fileDisabledMods;

        const badgeClass = `badge-${file.level.toLowerCase()}`;
        const levelLabel = file.level === 'Safe' ? t('tab_filter_safe') : file.level === 'Mergeable' ? t('tab_filter_mergeable') : t('tab_filter_conflict');
        let badgeHtml = '';
        if (isResolvedByToggle) {
            badgeHtml = `<span class="badge badge-resolved-toggle" title="${t('badge_resolved_toggle_title')}">${t('badge_resolved_toggle')}</span>`;
        } else {
            badgeHtml = `<span class="badge ${badgeClass}">${levelLabel}</span>`;
        }

        let resolutionBannerHtml = '';
        if (isMultiMod) {
            if (currentWinner) {
                if (isResolvedByToggle) {
                    resolutionBannerHtml = `
                        <div class="conflict-resolution-banner resolved-banner">
                            <div style="display:inline-flex; align-items:center; gap:8px;">
                                <span style="color:var(--color-safe); font-weight:600;">${t('priority_winner_label')}:</span>
                                <span class="pill-active-winner">${escapeHtml(currentWinner)}</span>
                                <span class="badge-resolved-tag">${t('badge_resolved_toggle')}</span>
                            </div>
                            <div style="display:inline-flex; align-items:center; gap:8px; font-size:0.8rem;">
                                <span style="color:var(--color-conflict); font-weight:600;">${t('overwritten_disabled_label')}:</span>
                                <span style="text-decoration:line-through; color:#d89696; opacity:0.8;">${escapeHtml(disabledMods.join(', '))}</span>
                            </div>
                        </div>
                    `;
                } else if (file.level === 'Mergeable') {
                    resolutionBannerHtml = `
                        <div class="conflict-resolution-banner mergeable-banner">
                            <div style="display:inline-flex; align-items:center; gap:8px;">
                                <span style="color:#8bb7cf; font-weight:600;">${t('mergeable_base_label')}:</span>
                                <span class="pill-active-winner">${escapeHtml(currentWinner)}</span>
                            </div>
                            ${otherActiveMods.length > 0 ? `
                                <div style="display:inline-flex; align-items:center; gap:8px;">
                                    <span style="color:#8bb7cf; font-weight:600;">${t('mergeable_combined_label')}:</span>
                                    <span class="pill-combined">${escapeHtml(otherActiveMods.join(', '))}</span>
                                </div>
                            ` : ''}
                            ${disabledMods.length > 0 ? `
                                <div style="display:inline-flex; align-items:center; gap:8px; font-size:0.8rem;">
                                    <span style="color:var(--color-conflict); font-weight:600;">${t('overwritten_disabled_label')}:</span>
                                    <span style="text-decoration:line-through; color:#d89696; opacity:0.8;">${escapeHtml(disabledMods.join(', '))}</span>
                                </div>
                            ` : ''}
                            <div style="font-size:0.78rem; color:#8a94a0; width:100%; margin-top:2px;">
                                ${t('mergeable_banner_hint')}
                            </div>
                        </div>
                    `;
                } else {
                    resolutionBannerHtml = `
                        <div class="conflict-resolution-banner">
                            <div style="display:inline-flex; align-items:center; gap:8px;">
                                <span style="color:var(--color-safe); font-weight:600;">${t('priority_winner_label')}:</span>
                                <span class="pill-active-winner">${escapeHtml(currentWinner)}</span>
                            </div>
                            ${(otherActiveMods.length > 0 || disabledMods.length > 0) ? `
                                <div style="display:inline-flex; align-items:center; gap:8px; font-size:0.8rem;">
                                    <span style="color:var(--color-conflict); font-weight:600;">${t('overwritten_mods_label')}:</span>
                                    <span style="text-decoration:line-through; color:#8a94a0;">${escapeHtml([...otherActiveMods, ...disabledMods].join(', '))}</span>
                                </div>
                            ` : ''}
                        </div>
                    `;
                }
            } else {
                resolutionBannerHtml = `
                    <div class="conflict-resolution-banner all-disabled-banner" style="background:rgba(196,64,64,0.08); border-left: 3px solid var(--color-conflict); border-color: rgba(196,64,64,0.25);">
                        <span style="color:var(--color-conflict); font-weight:600;">${t('all_mods_disabled_warning')}</span>
                    </div>
                `;
            }
        }

        // Checkbox Mod Selector
        let conflictSelectionHtml = '';
        if (isMultiMod) {
            conflictSelectionHtml = `
                <div class="conflict-selection-box">
                    <div class="conflict-selection-header">
                        <span class="conflict-selection-title">${t('conflict_choice_label')}</span>
                        <span class="conflict-selection-hint">${t('conflict_active_hint')}</span>
                    </div>
                    <div class="conflict-checkboxes-row">
                        ${file.present_in_mods.map(m => {
                            const encMod = encodeURIComponent(m);
                            const encPath = encodeURIComponent(file.relative_path);
                            const isChecked = !disabledMods.includes(m);
                            const isWinner = (m === currentWinner);

                            let cardClass = '';
                            let statusBadge = '';
                            if (!isChecked) {
                                cardClass = 'is-disabled';
                                statusBadge = `<span class="badge-disabled-tag">${t('toggle_badge_off')}</span>`;
                            } else if (file.level === 'Mergeable') {
                                if (isWinner) {
                                    cardClass = 'is-winner';
                                    statusBadge = `<span class="badge-active-tag">${t('mod_tag_active')}</span>`;
                                } else {
                                    cardClass = 'is-combined';
                                    statusBadge = `<span class="badge-combined-tag">${t('mod_tag_combined')}</span>`;
                                }
                            } else {
                                if (isWinner) {
                                    cardClass = 'is-winner';
                                    statusBadge = `<span class="badge-active-tag">${t('mod_tag_active')}</span>`;
                                } else {
                                    cardClass = 'is-overwritten';
                                    statusBadge = `<span class="badge-overwritten-tag">${t('mod_tag_overwritten')}</span>`;
                                }
                            }

                            return `
                                <label class="mod-checkbox-card ${cardClass}" title="${isChecked ? t('file_toggle_title_on') : t('file_toggle_title_off')}">
                                    <input type="checkbox" ${isChecked ? 'checked' : ''} 
                                           onchange="toggleFileInMod(decodeURIComponent('${encMod}'), decodeURIComponent('${encPath}'), event)">
                                    <span class="mod-checkbox-name">${escapeHtml(m)}</span>
                                    ${statusBadge}
                                </label>
                            `;
                        }).join('')}
                    </div>
                </div>
            `;
        } else {
            conflictSelectionHtml = `
                <div class="file-mods" style="margin-top: 8px;">
                    ${t('present_in')}: <span class="mod-pill"><span class="badge-active-tag" style="margin-right:4px;">${t('mod_tag_active')}</span>${escapeHtml(file.present_in_mods[0])}</span>
                </div>
            `;
        }

        let subItemsHtml = '';
        if (file.sub_items && file.sub_items.length > 0) {
            subItemsHtml = `
                <div class="sub-items-container">
                    <div class="sub-items-header">
                        <span>${t('details_discrepancies', { count: file.sub_items.length })}</span>
                    </div>
                    ${file.sub_items.map(sub => {
                        const subBadgeClass = `badge-${sub.level.toLowerCase()}`;
                        let valueBadgesHtml = '';

                        if (sub.mod_values && Object.keys(sub.mod_values).length > 0) {
                            const valBadges = [];
                            if (currentWinner && sub.mod_values[currentWinner] !== undefined) {
                                valBadges.push(`
                                    <div class="val-badge-active" title="${t('val_active_title')}">
                                        <span class="val-tag">${t('val_active_badge')}</span>
                                        <span class="val-text">${escapeHtml(currentWinner)}: ${escapeHtml(sub.mod_values[currentWinner])}</span>
                                    </div>
                                `);
                            }
                            file.present_in_mods.forEach(m => {
                                if (m !== currentWinner && sub.mod_values[m] !== undefined) {
                                    const isModOff = disabledMods.includes(m);
                                    valBadges.push(`
                                        <div class="val-badge-inactive" title="${t('val_inactive_title')}">
                                            <span class="val-tag">${isModOff ? t('val_inactive_badge') + ' (OFF)' : t('val_inactive_badge')}</span>
                                            <span class="val-text">${escapeHtml(m)}: ${escapeHtml(sub.mod_values[m])}</span>
                                        </div>
                                    `);
                                }
                            });
                            if (valBadges.length > 0) {
                                valueBadgesHtml = `
                                    <div class="sub-item-active-inactive-row">
                                        ${valBadges.join('')}
                                    </div>
                                `;
                            }
                        }

                        // Fallback if mod_values was not set
                        if (!valueBadgesHtml && (sub.active_val || sub.inactive_val)) {
                            const isDefaultWinner = !currentWinner || currentWinner === file.winner_mod;
                            const activeValText = isDefaultWinner ? sub.active_val : sub.inactive_val;
                            const inactiveValText = isDefaultWinner ? sub.inactive_val : sub.active_val;

                            valueBadgesHtml = `
                                <div class="sub-item-active-inactive-row">
                                    ${activeValText ? `
                                        <div class="val-badge-active" title="${t('val_active_title')}">
                                            <span class="val-tag">${t('val_active_badge')}</span>
                                            <span class="val-text">${escapeHtml(activeValText)}</span>
                                        </div>
                                    ` : ''}
                                    ${inactiveValText ? `
                                        <div class="val-badge-inactive" title="${t('val_inactive_title')}">
                                            <span class="val-tag">${t('val_inactive_badge')}</span>
                                            <span class="val-text">${escapeHtml(inactiveValText)}</span>
                                        </div>
                                    ` : ''}
                                </div>
                            `;
                        }

                        return `
                            <div class="sub-item-row" style="padding: 8px 10px; border-bottom: 1px solid rgba(255,255,255,0.05);">
                                <div style="display:flex; align-items:center; gap:8px; flex-wrap:wrap;">
                                    <span class="badge ${subBadgeClass}">${sub.level}</span>
                                    <span class="sub-item-name" style="font-weight:600; color:var(--text-primary);">${escapeHtml(sub.name)}</span>
                                    ${sub.detail ? `<span class="sub-item-detail" style="color:#94a3b8; font-size:0.8rem;">${escapeHtml(sub.detail)}</span>` : ''}
                                </div>
                                ${valueBadgesHtml}
                            </div>
                        `;
                    }).join('')}
                </div>
            `;
        }

        item.innerHTML = `
            <div class="file-header">
                <div class="file-path">
                    <span class="file-ext">[${escapeHtml(file.file_type)}]</span>
                    <strong>${escapeHtml(file.relative_path)}</strong>
                </div>
                ${badgeHtml}
            </div>
            <div class="file-summary">${escapeHtml(file.summary)}</div>
            ${resolutionBannerHtml}
            ${conflictSelectionHtml}
            ${subItemsHtml}
        `;

        list.appendChild(item);
    });
}

async function runMerge() {
    if (!currentScanData) {
        await showCustomAlert(t('alert_min_mods'), t('modal_title_info'));
        return;
    }

    const activeMods = currentModFolders.filter(m => isModEnabled(m.id));
    if (activeMods.length < 2) {
        await showCustomAlert(t('alert_min_mods_active'), t('modal_title_info'));
        return;
    }

    const selectedMode = document.querySelector('input[name="resolutionMode"]:checked').value;
    const btnMerge = document.getElementById('btnMerge');
    btnMerge.disabled = true;
    btnMerge.innerText = t('btn_merging');

    const modsToMerge = activeMods.map(m => ({
        name: m.name,
        path: m.relative_path,
        variant: m.selected_variant || null,
        disabled_files: disabledFilesState[m.name] || []
    }));

    try {
        const response = await fetch('/api/merge', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                mods: modsToMerge,
                resolution_mode: selectedMode,
                output_dir: ''
            })
        });

        if (!response.ok) {
            const err = await response.text();
            throw new Error(err || 'Failed to merge mods.');
        }

        const res = await response.json();
        let modeName = t('mode_smart_title');
        if (selectedMode === 'priority') modeName = t('mode_priority_title');
        if (selectedMode === 'manual') modeName = t('mode_manual_title');

        await showCustomAlert(
            t('alert_merge_result', {
                copied: res.files_copied,
                merged: res.files_merged,
                mode: modeName
            }),
            t('modal_title_success')
        );
        openFolder('merged');
        loadDeployerStatus();
    } catch (err) {
        await showCustomAlert(t('alert_err_generic', { msg: err.message }), t('modal_title_error'));
    } finally {
        btnMerge.disabled = false;
        btnMerge.innerText = t('btn_merge_all');
    }
}

function escapeHtml(str) {
    if (!str) return '';
    return str
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#039;');
}

/* ============================================================
   DEPLOYER & LAUNCHER (ZERO-OVERWRITE) LOGIC
   ============================================================ */

async function loadDeployerStatus() {
    try {
        const res = await fetch('/api/deployer/status');
        if (!res.ok) return;
        const data = await res.json();

        // 1. Update Game Path
        const inputGamePath = document.getElementById('inputGamePath');
        if (inputGamePath && (!inputGamePath.value || document.activeElement !== inputGamePath)) {
            inputGamePath.value = data.game_path;
        }

        // 2. Detection Tag
        const badge = document.getElementById('gameDetectedBadge');
        const badgeText = document.getElementById('gameDetectedText');
        if (badge && badgeText) {
            if (data.game_found) {
                badge.className = 'game-detected-tag detected';
                badgeText.innerText = t('exe_found');
            } else {
                badge.className = 'game-detected-tag missing';
                badgeText.innerText = t('exe_missing');
            }
        }

        // 2b. Launch Method Detection Badge
        const badgeMethod = document.getElementById('badgeDetectedMethod');
        if (badgeMethod) {
            if (data.is_steam_detected) {
                badgeMethod.className = 'badge-detected-method steam';
                badgeMethod.innerText = t('detected_steam_badge');
            } else {
                badgeMethod.className = 'badge-detected-method direct';
                badgeMethod.innerText = t('detected_direct_badge');
            }
        }

        // 3. Top Tab Status Pill
        const pill = document.getElementById('navLauncherStatusPill');
        if (pill) {
            if (data.is_modded) {
                pill.className = 'nav-status-pill pill-modded';
                pill.innerText = `${t('status_modded')} (${data.deployed_files_count})`;
            } else {
                pill.className = 'nav-status-pill pill-vanilla';
                pill.innerText = t('status_vanilla');
            }
        }

        // 4. Banner Mode
        const banner = document.getElementById('launcherStatusBanner');
        const bannerIcon = document.getElementById('bannerStatusIcon');
        const bannerTitle = document.getElementById('bannerStatusTitle');
        const bannerDesc = document.getElementById('bannerStatusDesc');
        const bannerMeta = document.getElementById('bannerStatusMeta');

        if (banner) {
            if (data.is_modded) {
                banner.className = 'launcher-status-banner banner-modded';
                if (bannerIcon) bannerIcon.innerHTML = '<svg class="banner-svg-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>';
                if (bannerTitle) bannerTitle.innerText = t('banner_modded_title');
                if (bannerDesc) bannerDesc.innerText = t('banner_modded_desc', { count: data.deployed_files_count });
                if (bannerMeta) bannerMeta.innerText = t('banner_active_meta', { time: data.deployed_at || 'Current session' });
            } else {
                banner.className = 'launcher-status-banner banner-vanilla';
                if (bannerIcon) bannerIcon.innerHTML = '<svg class="banner-svg-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>';
                if (bannerTitle) bannerTitle.innerText = t('banner_vanilla_title');
                if (bannerDesc) bannerDesc.innerText = t('banner_vanilla_desc');
                if (bannerMeta) bannerMeta.innerText = t('banner_ready_meta', { count: data.merged_files_count });
            }
        }

        // 5. Buttons Enable/Disable
        const btnDeploy = document.getElementById('btnDeployMods');
        const btnRestore = document.getElementById('btnRestoreVanilla');
        const btnLaunch = document.getElementById('btnLaunchGame');

        if (btnDeploy) {
            btnDeploy.disabled = !data.game_found || data.merged_files_count === 0;
            btnDeploy.textContent = data.is_modded ? t('btn_redeploy_mods') : t('btn_deploy_mods');
        }

        if (btnRestore) {
            btnRestore.disabled = !data.is_modded;
        }

        if (btnLaunch) {
            btnLaunch.disabled = !data.game_found;
        }

        const btnLaunchSeamless = document.getElementById('btnLaunchSeamless');
        if (btnLaunchSeamless) {
            btnLaunchSeamless.disabled = !data.game_found;
        }

        // 6. Active Mods in Fusion List
        const launcherModsCountPill = document.getElementById('launcherModsCountPill');
        const launcherActiveModsList = document.getElementById('launcherActiveModsList');

        let modsToDisplay = [];
        let isLiveDeployed = false;

        if (data.is_modded && data.active_mods && data.active_mods.length > 0) {
            modsToDisplay = data.active_mods;
            isLiveDeployed = true;
        } else if (currentModFolders.length > 0) {
            const activeMods = currentModFolders.filter(m => isModEnabled(m.id));
            modsToDisplay = activeMods.map(m => ({
                name: m.name,
                path: m.relative_path,
                variant: m.selected_variant || null
            }));
            isLiveDeployed = false;
        }

        if (launcherModsCountPill) {
            launcherModsCountPill.innerText = `${modsToDisplay.length} Mod(s)`;
        }

        if (launcherActiveModsList) {
            if (modsToDisplay.length === 0) {
                launcherActiveModsList.innerHTML = `<div style="color: #7d838a; font-size: 0.85rem; text-align: center; padding: 14px;">${t('no_mods_configured')}</div>`;
            } else {
                launcherActiveModsList.innerHTML = modsToDisplay.map((mod, idx) => {
                    const priorityClass = idx === 0 ? 'launcher-mod-priority first' : 'launcher-mod-priority';
                    const priorityText = idx === 0 ? '#1' : `#${idx + 1}`;
                    const variantHtml = mod.variant ? `<span class="launcher-mod-variant-tag">${t('variant_label')} ${escapeHtml(mod.variant)}</span>` : '';
                    const statusTag = isLiveDeployed
                        ? `<span class="launcher-mod-active-tag tag-deployed">${t('active_tag')}</span>`
                        : `<span class="launcher-mod-active-tag tag-staged">${t('staged_tag')}</span>`;

                    return `
                        <div class="launcher-mod-item">
                            <div class="launcher-mod-left">
                                <span class="${priorityClass}">${priorityText}</span>
                                <span class="launcher-mod-name">${escapeHtml(mod.name)}</span>
                                <span class="launcher-mod-folder">${escapeHtml(mod.path)}</span>
                            </div>
                            <div class="launcher-mod-right">
                                ${variantHtml}
                                ${statusTag}
                            </div>
                        </div>
                    `;
                }).join('');
            }
        }

        // 7. Deployed Files List
        const countSpan = document.getElementById('deployedFilesCount');
        const listDiv = document.getElementById('deployedFilesList');
        if (countSpan) countSpan.innerText = data.deployed_files_count;

        if (listDiv) {
            if (data.deployed_files.length === 0) {
                listDiv.innerHTML = `<div style="color: #7d838a; font-size: 0.85rem; text-align: center; padding: 18px;">${t('no_injected_files')}</div>`;
            } else {
                listDiv.innerHTML = data.deployed_files.map(f => {
                    const statusBadge = f.had_vanilla
                        ? `<span class="deployed-file-status deployed-status-backup">${t('badge_status_backup')}</span>`
                        : `<span class="deployed-file-status deployed-status-new">${t('badge_status_new')}</span>`;
                    return `
                        <div class="deployed-file-row">
                            <span class="deployed-file-path">${escapeHtml(f.relative_path)}</span>
                            ${statusBadge}
                        </div>
                    `;
                }).join('');
            }
        }
    } catch (e) {
        console.warn('Deployer status error:', e);
    }
}

async function deployMods() {
    const btn = document.getElementById('btnDeployMods');
    btn.disabled = true;
    btn.textContent = '...';

    const activeMods = currentModFolders.filter(m => isModEnabled(m.id)).map(m => ({
        name: m.name,
        path: m.relative_path,
        variant: m.selected_variant || null
    }));

    try {
        const res = await fetch('/api/deployer/deploy', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ active_mods: activeMods })
        });
        const data = await res.json();
        if (data.success) {
            await showCustomAlert(
                t('alert_deploy_success', { placed: data.placed_count, backup: data.backup_count }),
                t('modal_title_success')
            );
        } else {
            let errMsg = data.message;
            if (data.error_code === 'ERR_GAME_RUNNING') {
                errMsg = t('alert_err_game_running');
            } else if (data.error_code === 'ERR_EXE_NOT_FOUND') {
                errMsg = t('alert_err_exe_not_found', { path: data.detail || '' });
            } else if (data.error_code === 'ERR_MERGED_NOT_FOUND') {
                errMsg = t('alert_err_merged_not_found');
            } else if (data.error_code === 'ERR_MERGED_EMPTY') {
                errMsg = t('alert_err_merged_empty');
            }
            await showCustomAlert(errMsg || t('alert_err_deploy', { msg: '' }), t('modal_title_error'));
        }
    } catch (e) {
        await showCustomAlert(t('alert_err_deploy', { msg: e.message }), t('modal_title_error'));
    } finally {
        await loadDeployerStatus();
    }
}

async function restoreVanilla() {
    const confirmed = await showCustomConfirm(t('alert_restore_confirm'), t('modal_title_confirm'));
    if (!confirmed) {
        return;
    }
    const btn = document.getElementById('btnRestoreVanilla');
    btn.disabled = true;
    btn.textContent = '...';

    try {
        const res = await fetch('/api/deployer/restore', { method: 'POST' });
        const data = await res.json();
        if (data.success) {
            if (data.already_vanilla) {
                await showCustomAlert(t('alert_already_vanilla'), t('modal_title_info'));
            } else {
                await showCustomAlert(
                    t('alert_restore_success', { removed: data.removed_count, restored: data.restored_count }),
                    t('modal_title_success')
                );
            }
        } else {
            let errMsg = data.message;
            if (data.error_code === 'ERR_GAME_RUNNING') {
                errMsg = t('alert_err_game_running');
            }
            await showCustomAlert(errMsg || t('alert_err_restore', { msg: '' }), t('modal_title_error'));
        }
    } catch (e) {
        await showCustomAlert(t('alert_err_restore', { msg: e.message }), t('modal_title_error'));
    } finally {
        btn.textContent = t('btn_restore_vanilla');
        await loadDeployerStatus();
    }
}

async function launchGame(modeOverride = null) {
    const modeSelect = document.getElementById('selectLaunchMode');
    const selectedMode = modeOverride || (modeSelect ? modeSelect.value : 'auto');

    try {
        const res = await fetch('/api/deployer/launch', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ mode: selectedMode })
        });
        const data = await res.json();
        if (data.success) {
            let successMsg = t('alert_launch_direct_success');
            if (data.launch_mode === 'steam') successMsg = t('alert_launch_steam_success');
            if (data.launch_mode === 'seamless') successMsg = t('alert_launch_seamless_success');
            await showCustomAlert(successMsg, t('modal_title_success'));
        } else {
            await showCustomAlert(data.message || t('alert_err_launch', { msg: '' }), t('modal_title_error'));
        }
    } catch (e) {
        await showCustomAlert(t('alert_err_launch', { msg: e.message }), t('modal_title_error'));
    }
}

async function saveGamePath() {
    const input = document.getElementById('inputGamePath');
    if (!input) return;
    const path = input.value.trim();
    if (!path) {
        await showCustomAlert(t('alert_invalid_path'), t('modal_title_error'));
        return;
    }
    try {
        const res = await fetch('/api/deployer/set_path', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ path })
        });
        if (res.ok) {
            await loadDeployerStatus();
            await showCustomAlert(t('alert_game_path_saved'), t('modal_title_success'));
        }
    } catch (e) {
        await showCustomAlert(t('alert_err_generic', { msg: e.message }), t('modal_title_error'));
    }
}

async function openGameFolder() {
    try {
        await fetch('/api/deployer/open_game_folder', { method: 'POST' });
    } catch (e) {
        await showCustomAlert(t('alert_err_explorer'), t('modal_title_error'));
    }
}
