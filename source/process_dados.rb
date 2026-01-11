#!/usr/bin/env ruby

require 'csv'
require 'json'

# Estrutura para armazenar os dados
dados = {}

# Ler arquivos de 2016 a 2024
(2016..2024).each do |ano|
  arquivo = "#{ano}tpatendimento.csv"
  
  unless File.exist?(arquivo)
    puts "Aviso: Arquivo #{arquivo} não encontrado"
    next
  end
  
  puts "Processando #{arquivo}..."
  
  # Ler o arquivo CSV
  CSV.foreach(arquivo, col_sep: ';', headers: true) do |row|
    # Pular linhas vazias ou nulas
    next if row[0].nil? || row[0].strip.empty?
    next if row[1].nil? || row[1].strip.empty?
    
    distrito = row[0].strip
    tempo = row[1].strip.to_i
    
    # Criar entrada para o distrito se não existir
    dados[distrito] ||= {}
    
    # Adicionar o tempo de atendimento para este ano
    dados[distrito][ano.to_s] = tempo
  end
end

# Ordenar distritos alfabeticamente
dados = dados.sort.to_h

# Gerar arquivo JSON
File.write('dados_consolidados.json', JSON.pretty_generate(dados))

puts "\nArquivo 'dados_consolidados.json' gerado com sucesso!"
puts "Total de distritos: #{dados.keys.length}"
