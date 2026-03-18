use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;

/// Main application state for the Steam Ticket Generator GUI
struct TicketGeneratorApp {
    /// Input: App ID
    app_id_input: String,
    /// Status messages and logs
    status_log: String,
    /// Generated Steam ID (if successful)
    steam_id: Option<u64>,
    /// Generated ticket (base64 encoded)
    ticket: Option<String>,
    /// Whether generation is in progress
    is_generating: bool,
    /// Shared state for background task
    generation_state: Arc<Mutex<GenerationState>>,
}

/// State shared between GUI thread and background generation thread
#[derive(Clone)]
struct GenerationState {
    is_running: bool,
    result: Option<Result<(u64, String), String>>,
}

impl Default for TicketGeneratorApp {
    fn default() -> Self {
        Self {
            app_id_input: String::new(),
            status_log: String::from("Ready. Enter an App ID and click Generate.\n"),
            steam_id: None,
            ticket: None,
            is_generating: false,
            generation_state: Arc::new(Mutex::new(GenerationState {
                is_running: false,
                result: None,
            })),
        }
    }
}

impl TicketGeneratorApp {
    /// Validates the App ID input
    fn validate_app_id(&self) -> Result<u32, String> {
        if self.app_id_input.trim().is_empty() {
            return Err("App ID cannot be empty".to_string());
        }

        self.app_id_input
            .trim()
            .parse::<u32>()
            .map_err(|_| "App ID must be a valid number".to_string())
    }

    /// Starts ticket generation in a background thread
    fn start_generation(&mut self, app_id: u32) {
        self.is_generating = true;
        self.status_log.push_str(&format!("Starting generation for App ID: {}\n", app_id));
        self.status_log.push_str("Initializing Steam API...\n");

        let state = Arc::clone(&self.generation_state);

        // Reset previous result
        {
            let mut gen_state = state.lock().unwrap();
            gen_state.is_running = true;
            gen_state.result = None;
        }

        // Spawn background thread for ticket generation
        thread::spawn(move || {
            let result = generate_ticket_core(app_id);

            let mut gen_state = state.lock().unwrap();
            gen_state.is_running = false;
            gen_state.result = Some(result);
        });
    }

    /// Checks if background generation has completed and updates UI
    fn check_generation_status(&mut self) {
        let mut gen_state = self.generation_state.lock().unwrap();

        if !gen_state.is_running {
            if let Some(result) = gen_state.result.take() {
                self.is_generating = false;

                match result {
                    Ok((steam_id, ticket)) => {
                        self.steam_id = Some(steam_id);
                        self.ticket = Some(ticket.clone());
                        self.status_log.push_str(&format!("✓ Success!\n"));
                        self.status_log.push_str(&format!("Steam ID: {}\n", steam_id));
                        self.status_log.push_str(&format!("Ticket: {}...\n", &ticket[..ticket.len().min(50)]));
                        self.status_log.push_str("You can now save the ticket to a file.\n");
                    }
                    Err(e) => {
                        self.status_log.push_str(&format!("✗ Error: {}\n", e));
                        self.status_log.push_str("Make sure Steam is running and you're logged in.\n");
                        self.steam_id = None;
                        self.ticket = None;
                    }
                }
            }
        }
    }

    /// Saves the generated ticket to a file
    fn save_ticket(&mut self) {
        if let (Some(steam_id), Some(ticket)) = (self.steam_id, &self.ticket) {
            match create_config_file(steam_id, ticket) {
                Ok(_) => {
                    self.status_log.push_str("✓ Saved to configs.user.ini\n");
                }
                Err(e) => {
                    self.status_log.push_str(&format!("✗ Failed to save: {}\n", e));
                }
            }
        }
    }
}

impl eframe::App for TicketGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if background generation has completed
        if self.is_generating {
            self.check_generation_status();
            ctx.request_repaint(); // Keep updating while generating
        }

        // Main panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Steam Ticket Generator");
            ui.add_space(10.0);

            // Input section
            ui.group(|ui| {
                ui.label("Configuration:");
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("App ID:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.app_id_input)
                            .hint_text("e.g., 480")
                            .desired_width(200.0),
                    );
                });
            });

            ui.add_space(10.0);

            // Generate button
            ui.horizontal(|ui| {
                let button = egui::Button::new(if self.is_generating {
                    "⟳ Generating..."
                } else {
                    "Generate Ticket"
                });

                if ui.add_sized([150.0, 30.0], button).clicked() && !self.is_generating {
                    match self.validate_app_id() {
                        Ok(app_id) => {
                            self.start_generation(app_id);
                        }
                        Err(e) => {
                            self.status_log.push_str(&format!("✗ Validation error: {}\n", e));
                        }
                    }
                }

                // Save button (only enabled if ticket is generated)
                ui.add_enabled_ui(self.ticket.is_some() && !self.is_generating, |ui| {
                    if ui.add_sized([150.0, 30.0], egui::Button::new("💾 Save to File")).clicked() {
                        self.save_ticket();
                    }
                });
            });

            ui.add_space(10.0);

            // Status/log section
            ui.group(|ui| {
                ui.label("Status & Output:");
                ui.add_space(5.0);

                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.status_log.as_str())
                                .desired_width(f32::INFINITY)
                                .font(egui::TextStyle::Monospace),
                        );
                    });
            });

            ui.add_space(10.0);

            // Clear button
            if ui.button("Clear Log").clicked() {
                self.status_log.clear();
                self.status_log.push_str("Ready. Enter an App ID and click Generate.\n");
            }

            ui.add_space(5.0);
            ui.separator();
            ui.small("Note: Steam client must be running and logged in.");
        });
    }
}

// ============================================================================
// Core ticket generation logic (integrates with existing code)
// ============================================================================

/// Core function to generate the encrypted app ticket
/// Returns (Steam ID, Base64-encoded ticket) on success
fn generate_ticket_core(app_id: u32) -> Result<(u64, String), String> {
    use base64::{prelude::BASE64_STANDARD, Engine as _};
    use std::time::Duration;
    use steamworks_sys::ESteamAPIInitResult;

    unsafe {
        // Set environment variables for Steam API
        std::env::set_var("SteamAppId", &app_id.to_string());
        std::env::set_var("SteamGameId", app_id.to_string());

        // Initialize Steam API
        let init_result = steamworks_sys::SteamAPI_InitFlat(std::ptr::null_mut());
        steamworks_sys::SteamAPI_ManualDispatch_Init();

        match init_result {
            ESteamAPIInitResult::k_ESteamAPIInitResult_FailedGeneric => {
                return Err("Failed to initialize Steam API".to_string());
            }
            ESteamAPIInitResult::k_ESteamAPIInitResult_NoSteamClient => {
                return Err("Steam client is not running".to_string());
            }
            _ => {}
        }

        // Get Steam user interface
        let user = steamworks_sys::SteamAPI_SteamUser_v023();

        // Request encrypted app ticket
        steamworks_sys::SteamAPI_ISteamUser_RequestEncryptedAppTicket(
            user,
            std::ptr::null_mut(),
            0,
        );

        // Wait for the ticket callback
        let pipe = steamworks_sys::SteamAPI_GetHSteamPipe();
        loop {
            match run_callbacks(pipe) {
                Some(res) => {
                    if res != steamworks_sys::EResult::k_EResultOK {
                        return Err(format!("Failed to get encrypted app ticket, error: {:?}", res));
                    }
                    break;
                }
                None => {
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }

        // Retrieve the encrypted app ticket
        let ticket = {
            let mut ticket = vec![0; 2028];
            let mut ticket_len = 0;
            let success = steamworks_sys::SteamAPI_ISteamUser_GetEncryptedAppTicket(
                user,
                ticket.as_mut_ptr() as *mut _,
                2048,
                &mut ticket_len,
            );

            if !success {
                return Err("Failed to get encrypted app ticket. Does the account own the game?".to_string());
            }

            ticket.truncate(ticket_len as usize);
            BASE64_STANDARD.encode(&ticket)
        };

        // Get Steam ID
        let steamid = steamworks_sys::SteamAPI_ISteamUser_GetSteamID(user);

        Ok((steamid, ticket))
    }
}

/// Process Steam API callbacks
fn run_callbacks(pipe: i32) -> Option<steamworks_sys::EResult> {
    unsafe {
        let mut call = None;

        steamworks_sys::SteamAPI_ManualDispatch_RunFrame(pipe);
        let mut callback = std::mem::zeroed();

        while steamworks_sys::SteamAPI_ManualDispatch_GetNextCallback(pipe, &mut callback) {
            if callback.m_iCallback == steamworks_sys::SteamAPICallCompleted_t_k_iCallback as i32 {
                let apicall =
                    &mut *(callback.m_pubParam as *mut _ as *mut steamworks_sys::SteamAPICallCompleted_t);
                let mut apicall_result = vec![0; apicall.m_cubParam as usize];
                let mut failed = false;

                if steamworks_sys::SteamAPI_ManualDispatch_GetAPICallResult(
                    pipe,
                    apicall.m_hAsyncCall,
                    apicall_result.as_mut_ptr() as *mut _,
                    apicall.m_cubParam as _,
                    apicall.m_iCallback,
                    &mut failed,
                ) {
                    if !failed
                        && apicall.m_iCallback
                            == steamworks_sys::EncryptedAppTicketResponse_t_k_iCallback as i32
                    {
                        let res = &*(apicall_result.as_ptr()
                            as *const steamworks_sys::EncryptedAppTicketResponse_t);
                        call = Some(res.m_eResult);
                    }
                }
            }

            steamworks_sys::SteamAPI_ManualDispatch_FreeLastCallback(pipe);
        }

        call
    }
}

/// Creates the config file with Steam ID and ticket
fn create_config_file(steamid: u64, ticket: &str) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = std::fs::File::create("configs.user.ini")?;

    writeln!(file, "[user::general]")?;
    writeln!(file, "account_steamid={}", steamid)?;
    writeln!(file, "ticket={}", ticket)?;

    Ok(())
}

// ============================================================================
// Application entry point
// ============================================================================

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([500.0, 400.0])
            .with_icon(
                // You can add an icon here if you have one
                eframe::icon_data::from_png_bytes(&[]).unwrap_or_default(),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "Steam Ticket Generator",
        options,
        Box::new(|_cc| Ok(Box::<TicketGeneratorApp>::default())),
    )
}
