desc "re-generate this site."
task :default => :generate

task :g => :generate
task :generate do
  sh "bundle exec jekyll"
end

desc "start a local server to preview this site."
task :s => :server
task :server do
  sh "bundle exec jekyll --server --base-url --auto"
end

desc "create a new post file to _post"
task :n => :new_post
task :new_post do
  title = ENV['t'] || ENV['T'] || ENV['TITLE'] || 'untitled'
  date_str = Time.now.strftime('%Y-%m-%d')
  path = File.dirname(__FILE__) + "/_posts/#{date_str}-#{title}.md"
  open(path, 'w+') do |f|
    f.write %{
      ---
      layout: post
      title: "#{title}"
      ---
    }.strip.split("\n").map{|s| s.strip }.join("\n")
  end
  puts "new file: #{path}"
end

desc "open the last blog post in gvim"
task :last do
  sh "gnome-open #{Dir['_posts/*.md'].last}"
end
