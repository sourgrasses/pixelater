require "gtk3"
require "helix_runtime"
require "pixelater/native"

class PixApp < Gtk::Application
  def initialize
    super("org.jenn.pixelater", :flags_none)

    signal_connect "activate" do |app|
      win = Gtk::ApplicationWindow.new(app)
      win.set_title("pixelater 0.1")

      opener = Gtk::FileChooserDialog.new(:title => "Open File",
                                          :buttons => [[Gtk::Stock::CANCEL, Gtk::ResponseType::CANCEL],
                                                       [Gtk::Stock::OPEN, Gtk::ResponseType::ACCEPT]])
      saver = Gtk::FileChooserDialog.new(:title => "Save File",
                                         :action => :save,
                                         :buttons => [[Gtk::Stock::CANCEL, Gtk::ResponseType::CANCEL],
                                                      [Gtk::Stock::SAVE, Gtk::ResponseType::ACCEPT]])

      image = Gtk::Image.new
      filename = ""
      selection = PixelSelection.new

      grid = Gtk::Grid.new
      grid.insert_row(0)
      grid.insert_row(0)

      toolbar = Gtk::Toolbar.new

      open_icon = Gtk::Image.new(:stock => Gtk::Stock::OPEN, :size => :dialog)
      save_icon = Gtk::Image.new(:stock => Gtk::Stock::FLOPPY, :size => :dialog)
      quit_icon = Gtk::Image.new(:stock => Gtk::Stock::QUIT, :size => :dialog)
      open_button = Gtk::ToolButton.new(:icon_widget => open_icon)
      save_button = Gtk::ToolButton.new(:icon_widget => save_icon)
      quit_button = Gtk::ToolButton.new(:icon_widget => quit_icon)
      minus_button = Gtk::ToolButton.new(:label => "-")
      plus_button = Gtk::ToolButton.new(:label => "+")

      open_button.signal_connect("clicked") {
        res = opener.run
        if (res == Gtk::ResponseType::ACCEPT)
          filename = opener.filename
          image.set_file(filename)
          opener.destroy
        elsif (res == Gtk::ResponseType::CANCEL)
          opener.destroy
        end
      }

      save_button.signal_connect("clicked") { 
        res = saver.run
        if (res == Gtk::ResponseType::ACCEPT)
          filename = saver.filename
          selection.save(filename)
          saver.destroy
        elsif (res == Gtk::ResponseType::CANCEL)
          saver.destroy
        end
      }

      quit_button.signal_connect("clicked") {
        File.unlink("tmp.jpg")
        win.destroy
      }

      toolbar.insert(open_button, 0)
      toolbar.insert(save_button, 1)
      toolbar.insert(minus_button, 2)
      toolbar.insert(plus_button, 3)
      toolbar.insert(quit_button, 4)

      grid.attach(toolbar, 0, 0, 1, 1)

      event_box = Gtk::EventBox.new
      event_box.add_child(image)

      event_box.signal_connect("button_press_event") do |w, event|
        if (event.button == 1)
          selection.set_start(event.x.round, event.y.round)
        end
      end

      event_box.signal_connect("button_release_event") do |w, event|
        if (event.button == 1)
          selection.set_size(event.x.round, event.y.round)
          selection.pixelate(filename, 2)

          image.set_file("tmp.jpg")
        end
      end

      grid.attach(event_box, 0, 1, 1, 1)

      win.add(grid)
      win.set_default_size(600, 600)

      win.show_all
    end
  end
end

app = PixApp.new
app.run
